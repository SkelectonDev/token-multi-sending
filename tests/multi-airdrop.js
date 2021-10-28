const anchor = require("@project-serum/anchor");
const { Token, TOKEN_PROGRAM_ID } = require("@solana/spl-token");
const Wallet = '@project-serum/sol-wallet-adapter';

const assert = require("assert");
// const {
//   TOKEN_PROGRAM_ID,
//   sleep,
//   getTokenAccount,
//   createMint,
//   createTokenAccount,

// } = require("./utils");

const {
  sleep,
  getTokenAccount,
  createMint,
  createMintInstructions,
  createTokenAccount,
  mintToAccount,
  findOrCreateAssociatedTokenAccount,
  bnToDecimal,
  newAccount,
  wrapSol,
} = require("./utils")

describe("airdrop", () => {
  //const provider = anchor.Provider.local();
//  anchor.setProvider(anchor.Provider.env());
  
//  let nina = anchor.workspace.Nina;
  let provider = anchor.Provider.env();


  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.MultiAirdrop;

  //watermelon : ERC20 token
  // All mints default to 6 decimal places.
  //  const watermelonAmount = new anchor.BN(5000000);
  const watermelonAmount = new anchor.BN(500);

  // These are all of the variables we assume exist in the world already and
  // are available to the client.
  let sender = null;
  let taker1 = null;
  let airdrop = null;
  let airdropMint = null;
  let airdropSigner = null;
  let watermelonMintToken = null;
  let watermelonMint = null;
  let pool_token = null;

  let ownerSolTokenAccount = null;
  let royaltySolTokenAccount = null;
  let senderSolTokenAccount = null;
  
  let creatorWatermelonTokenAccount = null;
  let taker1WatermelonTokenAccount = null;
  let taker2WatermelonTokenAccount = null;
  
  let wrappedSolMint = new anchor.web3.PublicKey('So11111111111111111111111111111111111111112');


  it("Initializes the state-of-the-world", async () => {
    owner = await newAccount(provider);
    sender = await newAccount(provider);
    taker1 = await newAccount(provider);
    taker2 = await newAccount(provider);
    
    watermelonMint = await createMint(provider);
    //watermelonMint = watermelonMintToken.publicKey;
    const [_wrappedSolTokenAccount, wrappedSolTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      provider.wallet.publicKey,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      wrappedSolMint,
      true,
    );
    ownerSolTokenAccount = _wrappedSolTokenAccount;

    const [_wrappedSenderSolTokenAccount, user1WrappedSolTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      sender.publicKey,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      wrappedSolMint,
      true,
    );
    senderSolTokenAccount = _wrappedSenderSolTokenAccount;

    let [_senderTokenAccount, senderTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      sender.publicKey,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      watermelonMint,
      true,
    );
    creatorWatermelonTokenAccount = _senderTokenAccount;


    let [_takerTokenAccount, takerTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      taker1.publicKey,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      watermelonMint,
      true
    );
    taker1WatermelonTokenAccount = _takerTokenAccount;

    let [_taker2TokenAccount, taker2TokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      taker2.publicKey,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      watermelonMint,
      true
    );
    taker2WatermelonTokenAccount = _taker2TokenAccount;

    // const tx = new anchor.web3.Transaction();
    // tx.add(wrappedSolTokenAccountIx,user1WrappedSolTokenAccountIx,takerTokenAccountIx,senderTokenAccountIx);
    // await provider.send(tx, []);

    // Mint Watermelon tokens the will be distributed.
    await mintToAccount(
      provider,
      watermelonMint,
      creatorWatermelonTokenAccount,
      //new anchor.BN(100000000),
      watermelonAmount.toString(),
      provider.wallet.publicKey
    );
    

    creator_watermelon_account = await getTokenAccount(
      provider,
      creatorWatermelonTokenAccount
    );
    assert.ok(creator_watermelon_account.amount.eq(watermelonAmount));
  });

  // These are all variables the client will have to create to initialize the
  // IDO pool

  it("Initalize Airdrop", async () => {

    const paymentMint = wrappedSolMint;
    airdropMint = anchor.web3.Keypair.generate();
    const airdropMintIx = await createMintInstructions(
      provider,
      provider.wallet.publicKey,
      airdropMint.publicKey,
      0,
    );

    const [_airdrop, airdropBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("token_airdrop")),
        airdropMint.publicKey.toBuffer(),
      ],
      program.programId,
    );
    airdrop = _airdrop;

    const [_airdropSigner, airdropSignerBump] = await anchor.web3.PublicKey.findProgramAddress(
      [airdrop.toBuffer()],
      program.programId,
    );
    airdropSigner = _airdropSigner;

    let [_royaltyPoolTokenAccount, royaltyPoolTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      airdropSigner,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      watermelonMint,
      //true
    );
    pool_token = _royaltyPoolTokenAccount;


    let [_royaltyTokenAccount, royaltyTokenAccountIx] = await findOrCreateAssociatedTokenAccount(
      provider,
      airdropSigner,
      anchor.web3.SystemProgram.programId,
      anchor.web3.SYSVAR_RENT_PUBKEY,
      paymentMint,
    );
    royaltySolTokenAccount = _royaltyTokenAccount;
    
    bumps = {
      release: airdropBump,
      signer: airdropSignerBump,
    };
    await program.rpc.initializeAirdrop(
      bumps,
      new anchor.BN(25),
      {
        accounts: {
          airdrop,
          airdropSigner,
          airdropMint: airdropMint.publicKey,
          payer: provider.wallet.publicKey,
          authority: provider.wallet.publicKey,
          paymentMint,
          royaltySolTokenAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [airdropMint],
        instructions: [
          ...airdropMintIx,
          royaltyTokenAccountIx,
          royaltyPoolTokenAccountIx
        ],
      }
    );

    const releaseAfter = await program.account.airdropAccount.fetch(airdrop);
    //console.log("++++++++ fee : ",releaseAfter.airdropFee);
  });

  it("Send_tokens on transaction", async () => {
  
      amount = 10;
      const mintPublicKey = watermelonMint;
  //    console.log(watermelonMint);
      const mintToken = new Token(
        provider.connection,
        mintPublicKey,
        TOKEN_PROGRAM_ID,
        provider.wallet.publicKey // the wallet owner will pay to transfer and to create recipients associated token account if it does not yet exist.
      );
      
      const fromTokenAccount = await mintToken.getOrCreateAssociatedAccountInfo(
        sender.publicKey
      );
      const solAfterBalance = await provider.connection.getBalance(sender.publicKey);

      const destPublicKey = taker1.publicKey;
      //new web3.PublicKey(to);

      // Get the derived address of the destination wallet which will hold the custom token
      const associatedDestinationTokenAddr = await Token.getAssociatedTokenAddress(
        mintToken.associatedProgramId,
        mintToken.programId,
        mintPublicKey,
        taker1.publicKey
      );

      //const receiverAccount = await provider.connection.getAccountInfo(associatedDestinationTokenAddr);
      //const royalty = await provider.connection.getAccountInfo(taker1WatermelonTokenAccount);
      receiverAccount = await getTokenAccount(
        provider,
        associatedDestinationTokenAddr,
      );  
      
      // console.log(taker1.publicKey);
      // console.log(receiverAccount);      
      const instructions = [];  

      if (receiverAccount === null) {

        instructions.push(
          Token.createAssociatedTokenAccountInstruction(
            mintToken.associatedProgramId,
            mintToken.programId,
            mintPublicKey,
            associatedDestinationTokenAddr,
            destPublicKey,
            provider.wallet.publicKey
          )
        )

      }
      
      instructions.push(
        Token.createTransferInstruction(
          TOKEN_PROGRAM_ID,
          fromTokenAccount.address,
          associatedDestinationTokenAddr,
          provider.wallet.publicKey,
          [],
          amount
        )
      );

      const transaction = new anchor.web3.Transaction().add(...instructions);
      transaction.feePayer = provider.wallet.publicKey;
      transaction.recentBlockhash = (await provider.connection.getRecentBlockhash()).blockhash;
      

      // Sign transaction, broadcast, and confirm
      var signature = await anchor.web3.sendAndConfirmTransaction(
        provider.connection,
        transaction,
        [provider.wallet],
        { commitment: 'confirmed' }
      )
      // console.log(transaction.signatures);
      // const transactionSignature = await provider.connection.sendRawTransaction(
      //   transaction.serialize(),
      //   { skipPreflight: true }
      // );

      // var signature = await anchor.web3.sendAndConfirmTransaction(
      //   provider.connection,
      //   transaction,
      //   [sender.publicKey]
      // );

      // let signed = await Wallet.signTransaction(transaction);
      // let txid = await provider.connection.sendRawTransaction(signed.serialize());
      // await provider.connection.confirmTransaction(txid);
//    await provider.connection.confirmTransaction(transactionSignature);
      //console.log(signature);
      //await provider.send(transaction, []);

  });
  
  it("Send_token", async () => {
    //const multisig = anchor.web3.Keypair.generate();
    //const multisigSize = 70; // Big enough.
    // const [
    //   multiSigner,
    //   nonce,
    // ] = await anchor.web3.PublicKey.findProgramAddress(
    //   [multisig.publicKey.toBuffer()],
    //   program.programId
    // );
//     creators_watermelon_account = await getTokenAccount(
//       provider,
//       creatorWatermelonTokenAccount
//     );
//     taker1_watermelon_account = await getTokenAccount(
//       provider,
//       taker1WatermelonTokenAccount
//     );
//     taker2_watermelon_account = await getTokenAccount(
//       provider,
//       taker2WatermelonTokenAccount
//     );
//     let accounts_list = [
//       {
//         mint : taker1_watermelon_account.mint,
//       owner : taker1_watermelon_account.owner,
//       amount : new anchor.BN(taker1_watermelon_account.amount)
//      },
//     ];

//      multiSender = anchor.web3.Keypair.generate();
//     // owner1 = anchor.web3.Keypair.generate();
//     // multiSender1 = anchor.web3.Keypair.generate();
//     // accounts_list = [owner1.publicKey,multiSender1.publicKey];
//     // accounts[1] = taker2_watermelon_account;
//     await program.rpc.sendToken(
//       accounts_list,
//       {
//         accounts: {
//           payer : sender.publicKey,
//           distributionAuthority : sender.publicKey,
//           creatorTokenAccount : creatorWatermelonTokenAccount,
//           receiverAuthority : taker1.publicKey,
//           takerTokenAccount : taker1WatermelonTokenAccount,
//           multiSender: multiSender.publicKey,
//           tokenProgram: TOKEN_PROGRAM_ID,
//           systemProgram: anchor.web3.SystemProgram.programId,
//           rent: anchor.web3.SYSVAR_RENT_PUBKEY
//         },
//         signers: [sender, taker1],
//         // instructions: [
//         //   await program.account.airdropAccount.createInstruction(
//         //     multisig,
//         //     multisigSize
//         //   ),
// //        ]
//       }
//     );

//     creators_watermelon_account = await getTokenAccount(
//       provider,
//       creatorWatermelonTokenAccount
//     );
//     assert.ok(creators_watermelon_account.amount.eq(new anchor.BN(480)));

//     receiver_watermelon_account = await getTokenAccount(
//       provider,
//       taker1WatermelonTokenAccount
//     );

//     assert.ok(receiver_watermelon_account.amount.eq(new anchor.BN(20)));
  });

  it("Send Wrap SOL", async () => {

    const solBeforeBalance = await provider.connection.getBalance(sender.publicKey);
    let airdrop_fee = 500000000;
    const {instructions, signers} = await wrapSol(
      provider,
      sender,
      new anchor.BN(airdrop_fee),
    );
    console.log("before sol : ",solBeforeBalance);
    await program.rpc.sendWrapSol(
      new anchor.BN(airdrop_fee), {
        accounts: {
          airdrop,
          airdropMint: airdropMint.publicKey,
          airdropSigner,
          payer: sender.publicKey,
          payerTokenAccount: signers[0].publicKey,
          poolSol: royaltySolTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
         },
        signers: [sender, ...signers],
        instructions: [
         ...instructions,
        ],
      }
    );

    const solAfterBalance = await provider.connection.getBalance(sender.publicKey);
    const royaltyTokenAccountAfter = await getTokenAccount(
      provider,
      royaltySolTokenAccount,
    );
    console.log("after sol : " + solAfterBalance + "after royalty: " + royaltyTokenAccountAfter.amount.toNumber());
 

    assert.equal(solAfterBalance, solBeforeBalance - airdrop_fee);

    //const airdropAfter = await program.account.airdrop_account.fetch(airdrop);
    assert.equal(royaltyTokenAccountAfter.amount.toNumber(), airdrop_fee);
  });

});
