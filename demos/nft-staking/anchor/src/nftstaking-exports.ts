// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import NftstakingIDL from '../target/idl/nftstaking.json'
import type { Nftstaking } from '../target/types/nftstaking'

// Re-export the generated IDL and type
export { Nftstaking, NftstakingIDL }

// The programId is imported from the program IDL.
export const NFTSTAKING_PROGRAM_ID = new PublicKey(NftstakingIDL.address)

// This is a helper function to get the Nftstaking Anchor program.
export function getNftstakingProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...NftstakingIDL, address: address ? address.toBase58() : NftstakingIDL.address } as Nftstaking, provider)
}

// This is a helper function to get the program ID for the Nftstaking program depending on the cluster.
export function getNftstakingProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Nftstaking program on devnet and testnet.
      return new PublicKey('5fF9fccWZZJZV19bimi6dJyBm3rbZGG4u68Y9GSiDrz2')
    case 'mainnet-beta':
    default:
      return NFTSTAKING_PROGRAM_ID
  }
}
