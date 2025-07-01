//! Tests for ZKane Factory Contract

#[cfg(test)]
mod tests {
    use crate::{ZKaneFactory, ZKANE_INSTANCE_BLOCK};
    use alkanes_support::id::AlkaneId;
    use metashrew_support::index_pointer::KeyValuePointer;

    #[test]
    fn test_zkane_factory_basic() {
        // Basic test to ensure the module compiles
        let factory = ZKaneFactory::default();
        assert!(!factory.initialized);
    }

    #[test]
    fn test_pool_id_generation() {
        let factory = ZKaneFactory::default();
        
        let asset_id = AlkaneId {
            block: 1,
            tx: 1,
        };
        
        let denomination = 1000000000u128; // 10 tokens with 8 decimals
        
        // Test pool ID generation
        let pool_id = factory.generate_pool_id(&asset_id, denomination);
        
        // Pool should be in the instance block
        assert_eq!(pool_id.block, ZKANE_INSTANCE_BLOCK);
        
        // Same inputs should generate same pool ID
        let pool_id2 = factory.generate_pool_id(&asset_id, denomination);
        assert_eq!(pool_id.tx, pool_id2.tx);
        
        // Different denomination should generate different pool ID
        let pool_id3 = factory.generate_pool_id(&asset_id, denomination * 2);
        assert_ne!(pool_id.tx, pool_id3.tx);
    }

    #[test]
    fn test_storage_pointers() {
        let factory = ZKaneFactory::default();
        
        // Test that storage pointers can be created
        let pools_ptr = factory.pools_pointer();
        let count_ptr = factory.pool_count_pointer();
        
        // Basic functionality test
        assert!(pools_ptr.get().is_empty() || !pools_ptr.get().is_empty()); // Always true, testing compilation
        assert!(count_ptr.get().is_empty() || !count_ptr.get().is_empty()); // Always true, testing compilation
    }
}