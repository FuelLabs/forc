use fuel_core_storage::column::Column;
use fuel_core_types::{services::executor::StorageReadReplayEvent, tai64::Tai64};
use fuel_vm::{
    error::{InterpreterError, RuntimeError},
    fuel_storage::{StorageRead, StorageReadError, StorageSize, StorageWrite},
    fuel_types::BlockHeight,
    prelude::*,
    storage::{
        BlobData, ContractsAssetKey, ContractsAssets, ContractsAssetsStorage, ContractsRawCode,
        ContractsState, ContractsStateData, ContractsStateKey, InterpreterStorage,
        UploadedBytecodes,
    },
};
use fuels_core::types::U256;
use std::{cell::RefCell, collections::HashMap};

type InnerStorage = HashMap<Column, HashMap<Vec<u8>, Option<Vec<u8>>>>;

#[derive(Clone)]
pub struct ShallowStorage {
    pub block_height: BlockHeight,
    pub timestamp: Tai64,
    pub consensus_parameters_version: u32,
    pub state_transition_version: u32,
    pub coinbase: fuel_vm::prelude::ContractId,
    pub storage: RefCell<InnerStorage>,
}

impl ShallowStorage {
    pub fn initial_storage(reads: Vec<StorageReadReplayEvent>) -> InnerStorage {
        let mut storage: InnerStorage = HashMap::new();
        for read in reads {
            let column = Column::try_from(read.column).expect("Invalid column id in read event");
            storage
                .entry(column)
                .or_default()
                .insert(read.key, read.value);
        }
        storage
    }

    fn value_of_column(&self, column: Column, key: Vec<u8>) -> Option<Vec<u8>> {
        self.storage.borrow().get(&column)?.get(&key)?.clone()
    }

    fn replace_column(
        &self,
        column: Column,
        key: Vec<u8>,
        value: Option<Vec<u8>>,
    ) -> Option<Vec<u8>> {
        self.storage
            .borrow_mut()
            .entry(column)
            .or_default()
            .insert(key.clone(), value)?
    }
}

macro_rules! storage_rw {
    ($vm_type:ident, $convert_key:expr, $convert_value:expr, $convert_value_back:expr $(,)?) => {
        storage_rw!(
            $vm_type = $vm_type,
            $convert_key,
            $convert_value,
            $convert_value_back
        );
    };
    ($vm_type:ident = $core_column:ident, $convert_key:expr, $convert_value:expr, $convert_value_back:expr $(,)?) => {
        impl StorageSize<$vm_type> for ShallowStorage {
            fn size_of_value(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<Option<usize>, Self::Error> {
                tracing::debug!(
                    "{:?} size_of_value {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                );
                let head = self.value_of_column(Column::$core_column, $convert_key(key));
                Ok(head.map(|v| v.len()))
            }
        }

        impl StorageInspect<$vm_type> for ShallowStorage {
            type Error = Error;

            fn get(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<
                Option<
                    std::borrow::Cow<'_, <$vm_type as fuel_vm::fuel_storage::Mappable>::OwnedValue>,
                >,
                Self::Error,
            > {
                tracing::debug!(
                    "{} get {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                );
                let head = self.value_of_column(Column::$core_column, $convert_key(key));
                Ok(head.map($convert_value).map(std::borrow::Cow::Owned))
            }

            fn contains_key(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<bool, Self::Error> {
                tracing::debug!(
                    "{} contains_key {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                );
                let head = self.value_of_column(Column::$core_column, $convert_key(key));
                Ok(head.is_some())
            }
        }

        impl StorageRead<$vm_type> for ShallowStorage {
            fn read_exact(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
                offset: usize,
                buf: &mut [u8],
            ) -> Result<Result<usize, StorageReadError>, Self::Error> {
                tracing::debug!(
                    "{} read_exact {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key)),
                );
                let head = self.value_of_column(Column::$core_column, $convert_key(key));
                let Some(value) = head else {
                    return Ok(Err(StorageReadError::KeyNotFound));
                };

                if offset >= value.len() || offset.saturating_add(buf.len()) > value.len() {
                    return Ok(Err(StorageReadError::OutOfBounds));
                }

                buf.copy_from_slice(&value[offset..][..buf.len()]);
                Ok(Ok(value.len()))
            }

            fn read_zerofill(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
                offset: usize,
                buf: &mut [u8],
            ) -> Result<Result<usize, StorageReadError>, Self::Error> {
                tracing::debug!(
                    "{} read_zerofill {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key)),
                );
                let head = self.value_of_column(Column::$core_column, $convert_key(key));
                let Some(value) = head else {
                    return Ok(Err(StorageReadError::KeyNotFound));
                };

                if offset >= value.len() {
                    return Ok(Err(StorageReadError::OutOfBounds));
                }

                let src = &value[offset..];
                let (dst, rest) = buf.split_at_mut(buf.len().min(src.len()));
                dst.copy_from_slice(&src[..dst.len()]);
                rest.fill(0);

                Ok(Ok(value.len()))
            }

            fn read_alloc(
                &self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<Option<Vec<u8>>, Self::Error> {
                todo!(
                    "{} read_alloc {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                )
            }
        }

        impl StorageMutate<$vm_type> for ShallowStorage {
            fn replace(
                &mut self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
                value: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Value,
            ) -> Result<
                Option<<$vm_type as fuel_vm::fuel_storage::Mappable>::OwnedValue>,
                Self::Error,
            > {
                tracing::debug!(
                    "{} replace {} (value={value:?})",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                );
                Ok(self
                    .replace_column(
                        Column::$core_column,
                        $convert_key(key),
                        Some($convert_value_back(value)),
                    )
                    .map($convert_value))
            }

            fn take(
                &mut self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<
                Option<<$vm_type as fuel_vm::fuel_storage::Mappable>::OwnedValue>,
                Self::Error,
            > {
                tracing::debug!(
                    "{} take {}",
                    stringify!($core_column),
                    hex::encode(&$convert_key(key))
                );
                Ok(self
                    .replace_column(Column::$core_column, $convert_key(key), None)
                    .map($convert_value))
            }
        }

        impl StorageWrite<$vm_type> for ShallowStorage {
            fn write_bytes(
                &mut self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
                _buf: &[u8],
            ) -> Result<(), Self::Error> {
                todo!("write_bytes {key:?}")
            }

            fn replace_bytes(
                &mut self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
                _buf: &[u8],
            ) -> Result<Option<Vec<u8>>, Self::Error> {
                tracing::debug!("{} replace_bytes {key:?}", stringify!($core_column));
                Ok(self.replace_column(Column::$core_column, $convert_key(key), None))
            }

            fn take_bytes(
                &mut self,
                key: &<$vm_type as fuel_vm::fuel_storage::Mappable>::Key,
            ) -> Result<Option<Vec<u8>>, Self::Error> {
                todo!("take_bytes {key:?}")
            }
        }
    };
}

storage_rw!(
    ContractsRawCode,
    |key: &ContractId| -> Vec<u8> { (**key).to_vec() },
    |data| todo!("ContractsRawCode from bytes {data:?}"),
    |data| -> Vec<u8> { todo!("ContractsRawCode to bytes {data:?}") },
);
storage_rw!(
    ContractsState,
    |key: &ContractsStateKey| -> Vec<u8> { key.as_ref().into() },
    |data| { ContractsStateData(data.into()) },
    |data: &[u8]| -> Vec<u8> { data.to_vec() },
);
storage_rw!(
    ContractsAssets,
    |key: &ContractsAssetKey| -> Vec<u8> { key.as_ref().into() },
    |data| {
        assert_eq!(data.len(), 8);
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&data);
        u64::from_be_bytes(buffer)
    },
    |data: &u64| -> Vec<u8> { data.to_be_bytes().to_vec() },
);
storage_rw!(
    UploadedBytecodes,
    |key: &Bytes32| -> Vec<u8> { key.as_ref().into() },
    |data| todo!("UploadedBytecodes from bytes {data:?}"),
    |data| -> Vec<u8> { todo!("UploadedBytecodes to bytes {data:?}") },
);
storage_rw!(
    BlobData = Blobs,
    |key: &BlobId| -> Vec<u8> { key.as_ref().into() },
    |data| todo!("BlobData from bytes {data:?}"),
    |data| -> Vec<u8> { todo!("BlobData to bytes {data:?}") },
);

impl ContractsAssetsStorage for ShallowStorage {}

#[derive(Debug)]
pub enum Error {
    /// This block couldn't have been included
    InvalidBlock,
    /// The requested key is out of the available keyspace
    KeyspaceOverflow,
    /// Read offset too large, or buffer too small
    CannotRead,
}
impl From<Error> for RuntimeError<Error> {
    fn from(e: Error) -> Self {
        RuntimeError::Storage(e)
    }
}
impl From<Error> for InterpreterError<Error> {
    fn from(e: Error) -> Self {
        InterpreterError::Storage(e)
    }
}

impl InterpreterStorage for ShallowStorage {
    type DataError = Error;

    fn block_height(&self) -> Result<BlockHeight, Self::DataError> {
        Ok(self.block_height)
    }

    fn consensus_parameters_version(&self) -> Result<u32, Self::DataError> {
        Ok(self.consensus_parameters_version)
    }

    fn state_transition_version(&self) -> Result<u32, Self::DataError> {
        Ok(self.state_transition_version)
    }

    fn timestamp(
        &self,
        height: fuel_vm::fuel_types::BlockHeight,
    ) -> Result<fuel_vm::prelude::Word, Self::DataError> {
        match height {
            height if height > self.block_height => Err(Error::InvalidBlock),
            height if height == self.block_height => Ok(self.timestamp.0),
            height => {
                todo!("timestamp {height:?}");
            }
        }
    }

    fn block_hash(
        &self,
        block_height: fuel_vm::fuel_types::BlockHeight,
    ) -> Result<fuel_vm::prelude::Bytes32, Self::DataError> {
        // Block header hashes for blocks with height greater than or equal to current block height are zero (0x00**32).
        // https://docs.fuel.network/docs/specs/fuel-vm/instruction-set/#bhsh-block-hash
        if block_height >= self.block_height || block_height == Default::default() {
            Ok(Bytes32::zeroed())
        } else {
            todo!("block_hash {block_height:?}");
        }
    }

    fn coinbase(&self) -> Result<fuel_vm::prelude::ContractId, Self::DataError> {
        Ok(self.coinbase)
    }

    fn set_consensus_parameters(
        &mut self,
        _version: u32,
        _consensus_parameters: &fuel_vm::prelude::ConsensusParameters,
    ) -> Result<Option<fuel_vm::prelude::ConsensusParameters>, Self::DataError> {
        unreachable!("Cannot be called by a script");
    }

    fn set_state_transition_bytecode(
        &mut self,
        _version: u32,
        _hash: &fuel_vm::prelude::Bytes32,
    ) -> Result<Option<fuel_vm::prelude::Bytes32>, Self::DataError> {
        unreachable!("Cannot be called by a script");
    }

    fn contract_state_remove_range(
        &mut self,
        contract: &fuel_vm::prelude::ContractId,
        start_key: &fuel_vm::prelude::Bytes32,
        range: usize,
    ) -> Result<(), Self::DataError> {
        tracing::debug!("contract_state_remove_range {contract:?} {start_key:?}");

        let mut key = U256::from_big_endian(start_key.as_ref());
        let mut key_buffer = Bytes32::zeroed();

        for idx in 0..range {
            if idx != 0 {
                key = key.checked_add(1.into()).ok_or(Error::KeyspaceOverflow)?;
            }

            key.to_big_endian(key_buffer.as_mut());
            self.storage::<ContractsState>()
                .remove(&(contract, Bytes32::from_bytes_ref(&key_buffer)).into())?;
        }

        Ok(())
    }
}
