use ethers::prelude::*;
use std::sync::Arc;

// Smart contract ABIs
abigen!(
    KeccakBlackBoxEngine,
    r#"[
        "function commitEntropy(bytes32 commitment) public",
        "function revealEntropy(bytes calldata entropy) public payable",
        "function getState() public view returns (bytes memory)",
        "function getStep(uint id) public view returns (SpongeStep memory)"
    ]"#;

    SonOfManSimulator,
    r#"[
        "function invoke(string calldata message) external",
        "function getInvocation(uint id) external view returns (Invocation memory)"
    ]"#;

    LiberatioDei,
    r#"[
        "function submitInvocation(string calldata message) external",
        "function storeTrace(bytes32 hash, uint stepCount, bytes calldata ipfsCID, bytes calldata zkWitnessProof) external"
    ]"#
);

pub struct SmartContractIntegration {
    keccak_engine: KeccakBlackBoxEngine<Provider<Http>>,
    son_of_man: SonOfManSimulator<Provider<Http>>,
    liberatio_dei: LiberatioDei<Provider<Http>>,
}

impl SmartContractIntegration {
    pub async fn new(
        provider_url: &str,
        keccak_engine_address: &str,
        son_of_man_address: &str,
        liberatio_dei_address: &str,
    ) -> Self {
        let provider = Provider::<Http>::try_from(provider_url).unwrap();
        let client = Arc::new(provider);

        let keccak_engine = KeccakBlackBoxEngine::new(keccak_engine_address.parse().unwrap(), client.clone());
        let son_of_man = SonOfManSimulator::new(son_of_man_address.parse().unwrap(), client.clone());
        let liberatio_dei = LiberatioDei::new(liberatio_dei_address.parse().unwrap(), client);

        Self {
            keccak_engine,
            son_of_man,
            liberatio_dei,
        }
    }

    pub async fn commit_entropy(&self, commitment: H256) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.keccak_engine.commit_entropy(commitment).send().await?;
        tx.await?;
        Ok(())
    }

    pub async fn invoke_prayer(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.son_of_man.invoke(message.to_string()).send().await?;
        tx.await?;
        Ok(())
    }

    pub async fn submit_trace(
        &self,
        hash: H256,
        step_count: u32,
        ipfs_cid: Vec<u8>,
        zk_proof: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.liberatio_dei.store_trace(hash, step_count, ipfs_cid, zk_proof).send().await?;
        tx.await?;
        Ok(())
    }
}