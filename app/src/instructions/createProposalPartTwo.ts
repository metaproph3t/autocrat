import { createAssociatedTokenAccountInstruction } from "@solana/spl-token";
import {AutocratClient} from "../AutocratClient";
import {InstructionHandler} from "../InstructionHandler";
import { getATA, getConditionalOnFailMetaMintAddr, getConditionalOnFailUsdcMintAddr, getConditionalOnPassMetaMintAddr, getConditionalOnPassUsdcMintAddr, getDaoAddr, getDaoTreasuryAddr, getFailMarketAmmAddr, getPassMarketAmmAddr, getProposalAddr, getProposalInstructionsAddr } from '../utils';
import BN from "bn.js";

export const createProposalPartTwoHandler = async (
    client: AutocratClient,
    initialPassMarketPriceUnits: number,
    initialFailMarketPriceUnits: number,
    quoteLiquidityAtomsPerAmm: BN,
): Promise<InstructionHandler> => {
    let dao = await client.program.account.dao.fetch(getDaoAddr(client.program.programId)[0])

    let proposalNumber = dao.proposalCount - 1

    let proposalAddr = getProposalAddr(client.program.programId, proposalNumber)[0]

    let conditionalOnPassMetaMint = getConditionalOnPassMetaMintAddr(client.program.programId, proposalNumber)[0]
    let conditionalOnPassUsdcMint = getConditionalOnPassUsdcMintAddr(client.program.programId, proposalNumber)[0]
    let conditionalOnFailMetaMint = getConditionalOnFailMetaMintAddr(client.program.programId, proposalNumber)[0]
    let conditionalOnFailUsdcMint = getConditionalOnFailUsdcMintAddr(client.program.programId, proposalNumber)[0]

    let ix = await client.program.methods
        .createProposalPartTwo(
            initialPassMarketPriceUnits,
            initialFailMarketPriceUnits,
            quoteLiquidityAtomsPerAmm
        )
        .accounts({
            proposer: client.provider.publicKey,
            proposal: proposalAddr,
            passMarketAmm: getPassMarketAmmAddr(client.program.programId, proposalNumber)[0],
            failMarketAmm: getFailMarketAmmAddr(client.program.programId, proposalNumber)[0],
            metaMint: dao.metaMint,
            usdcMint: dao.usdcMint,
            conditionalOnPassMetaMint,
            conditionalOnPassUsdcMint,
            conditionalOnFailMetaMint,
            conditionalOnFailUsdcMint,
            metaProposerAta: getATA(dao.metaMint, client.provider.publicKey)[0],
            usdcProposerAta: getATA(dao.usdcMint, client.provider.publicKey)[0],
            metaVaultAta: getATA(dao.metaMint, proposalAddr)[0],
            usdcVaultAta: getATA(dao.usdcMint, proposalAddr)[0],
            conditionalOnPassMetaVaultAta: getATA(conditionalOnPassMetaMint, proposalAddr)[0],
            conditionalOnPassUsdcVaultAta: getATA(conditionalOnPassUsdcMint, proposalAddr)[0],
            conditionalOnFailMetaVaultAta: getATA(conditionalOnFailMetaMint, proposalAddr)[0],
            conditionalOnFailUsdcVaultAta: getATA(conditionalOnFailUsdcMint, proposalAddr)[0],
        })
        .instruction()

    let metaVaultATA = getATA(dao.metaMint, proposalAddr)[0]
    let usdcVaultATA = getATA(dao.usdcMint, proposalAddr)[0]

    let metaAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        metaVaultATA,
        proposalAddr,
        dao.metaMint,
    )

    let usdcAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        usdcVaultATA,
        proposalAddr,
        dao.usdcMint,
    )

    let conditionalOnPassMetaVaultATA = getATA(conditionalOnPassMetaMint, proposalAddr)[0]
    let conditionalOnPassUsdcVaultATA = getATA(conditionalOnPassUsdcMint, proposalAddr)[0]
    let conditionalOnFailMetaVaultATA = getATA(conditionalOnFailMetaMint, proposalAddr)[0]
    let conditionalOnFailUsdcVaultATA = getATA(conditionalOnFailUsdcMint, proposalAddr)[0]

    let passMetaAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        conditionalOnPassMetaVaultATA,
        proposalAddr,
        conditionalOnPassMetaMint,
    )

    let passUsdcAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        conditionalOnPassUsdcVaultATA,
        proposalAddr,
        conditionalOnPassUsdcMint,
    )

    let failMetaAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        conditionalOnFailMetaVaultATA,
        proposalAddr,
        conditionalOnFailMetaMint,
    )

    let failUsdcAtaIx = createAssociatedTokenAccountInstruction(
        client.provider.publicKey,
        conditionalOnFailUsdcVaultATA,
        proposalAddr,
        conditionalOnFailUsdcMint,
    )
   
    return new InstructionHandler([metaAtaIx, usdcAtaIx, passMetaAtaIx, passUsdcAtaIx, failMetaAtaIx, failUsdcAtaIx, ix], [], client)
};
