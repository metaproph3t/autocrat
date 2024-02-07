import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {
    AddressLookupTableAccount,
    Keypair,
    PublicKey,
    Signer,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction
} from "@solana/web3.js";
import {Autocrat as AutocratIDLType} from '../../target/types/autocrat';
// @ts-ignore
import * as IDL from '../../target/idl/autocrat.json';
// import {
//     getTxAccounts,
//     initializeMarketPair,
//     TxAccounts,
//     refreshMarketOrderIx,
//     removeMarketPair, OrdersAndParams
// } from "./rpc";
import BN from "bn.js";
import {addComputeUnits} from "./utils";
import { AUTOCRAT_LUTS, AUTOCRAT_PROGRAM_ID } from "./constants";

export class AutocratClient {
    public readonly provider: AnchorProvider;
    public readonly program: Program<AutocratIDLType>;
    public readonly luts: AddressLookupTableAccount[];

    constructor(
        provider: AnchorProvider,
        programId: PublicKey,
        luts: AddressLookupTableAccount[],
    ) {
        this.provider = provider
        this.program = new Program<AutocratIDLType>(IDL, programId, provider)
        this.luts = luts
    }

    public static async createClient(provider: AnchorProvider, programId?: PublicKey): Promise<AutocratClient> {
        const getLuts = () => Promise.all(
            AUTOCRAT_LUTS.map(lut => {
                return provider.connection
                    .getAddressLookupTable(lut)
                    .then((res) => res.value as AddressLookupTableAccount)
            })
        )

        const luts = await getLuts()

        return new AutocratClient(
            provider,
            programId || AUTOCRAT_PROGRAM_ID,
            luts as AddressLookupTableAccount[],
        )
    }

    
}
