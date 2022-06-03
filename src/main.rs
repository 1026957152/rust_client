
/*use std::error::Error;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{system_transaction, signature::{Keypair, Signature}};
*/
use solana_sdk::system_program;


use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster, EventContext};
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};

// The `accounts` and `instructions` modules are generated by the framework.
use my_solana_program::accounts as basic_2_accounts;
use my_solana_program::instruction as basic_2_instruction;
//use rust_client::my_solana_program::Counter;
use std::rc::Rc;
use std::time::Duration;
use rand::rngs::OsRng;
use std::str::FromStr;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account;

//use solana_client::rpc_client::RpcClient;

const URL: &str = "https://api.devnet.solana.com";



//const URL: &str = "https://api.testnet.solana.com";
fn main() {


    println!("Starting test...");

    // Wallet and cluster params.
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let url = Cluster::Custom(
       // "http://localhost:8899".to_string(),
       // "ws://127.0.0.1:8900".to_string(),

       "https://api.testnet.solana.com".to_string(),

     //  "https://api.devnet.solana.com".to_string(),
        "ws://api.devnet.solana.com:8900".to_string(),
    );

    // Client.
    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    // Run tests.
/*    composite(&client, opts.composite_pid)?;*/
  //  basic_2(&client, Pubkey::from_str("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS").unwrap());

    //basic_又contract端进行mint到给定(&client, Pubkey::from_str("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS").unwrap());
   //basic_又contract端进行mint到给定_使用钱包公钥(&client, Pubkey::from_str("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS").unwrap());

    basic_又contract端进行mint到给定_使用钱包公钥_mint固定起来(&client, Pubkey::from_str("2kx16UybH5HZkX1AimamBqqJ6mYstTgqR2jmVinV3GJS").unwrap());


  /*  basic_4(&client, opts.basic_4_pid)?;
    events(&client, opts.events_pid)?;*/

    // Success.

    println!("Hello, world!");













    let usdc_mint_account_pubkey = Pubkey::from_str("FSRvxBNrQWX2Fy2qvKMLL3ryEdRtE3PUTZBcdKwASZTU").unwrap();
    //USDC
    //  let usdc_mint_account_pubkey = Pubkey::from_str("BEcGFQK1T1tSu3kvHC17cyCkQ5dvXqAJ7ExB2bb5Do7a").unwrap();

    let wallet_address = Pubkey::from_str("GnJpWUj3BmbH4KtaHhs52pnknVB3QqPLN68WxRfju2Sh").unwrap();




    println!("create_fund_receiper_spl_token_token_account");
/*    let lamport_requirement =
        connection.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();
*/

    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let ix = spl_associated_token_account::create_associated_token_account(
        &payer.pubkey(),
        &wallet_address,
        &usdc_mint_account_pubkey,
        //   token_program,
    );

/*    let cpi_accounts = Transfer {
        from: self
            .initializer_deposit_token_account
            .to_account_info()
            .clone(),
        to: self.vault_account.to_account_info().clone(),
        authority: self.initializer.clone(),
    };
    CpiContext::new(self.token_program.clone(), cpi_accounts)
    token::transfer(
        ctx.accounts.into_transfer_to_pda_context(),
        ctx.accounts.escrow_account.initializer_amount,
    )?;*/


}


// Runs a client for examples/tutorial/basic-2.
//
// Make sure to run a localnet with the program deploy to run this example.
fn basic_2(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.
  let counter = Keypair::generate(&mut OsRng);


  //  let (counter, counter_authority_bump_seed) = Pubkey::find_program_address(&[b"seed"], &program.id());

    let (mint_authority, mint_authority_bump_seed) = Pubkey::find_program_address(&[b"seed"], &program.id());

    // `Initialize` parameters.
    let dummy_a = Keypair::generate(&mut OsRng);
    let dummy_b = Keypair::generate(&mut OsRng);

    let wallet_address = Keypair::generate(&mut OsRng);

    let authority = program.payer();



    let puppet_program_id = Pubkey::from_str("J5Na4U6kohvHroxoSsfqRYMDQwosaYgmu2TLLYezx7AQ").unwrap();


    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_address.pubkey(),
        &dummy_a.pubkey(),
        //   token_program,
    );


  //  let ata = Pubkey::from_str("8jtxPqpUV8oex35gNu5HkEjNySw4x64jLBYBmYYUft9y").unwrap();
 //   let dummy_a_key = Pubkey::from_str("EexCW6kpKiiQaK4fooG8UpfVxDHvAVxwPESyTvh1ocug").unwrap();


    //let ix = ;

    // Build and send a transaction.
    let token_program =spl_token::id();// &id;
    let string = b"foo";
    program
        .request()
      .instruction(anchor_client::solana_sdk::system_instruction::create_account(
            &program.payer(),
            &dummy_a.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap(),
            Mint::LEN  as u64,
            &anchor_spl::token::ID,
        ))
        .instruction(spl_token::instruction::initialize_mint(
            &token_program,
            &dummy_a.pubkey(),
            &authority,  // 这里是mint 权限的人
            None,
            0,
        ).unwrap())

       .instruction(spl_associated_token_account::create_associated_token_account(
           &authority,
           &wallet_address.pubkey(),
           &dummy_a.pubkey(),
           //   token_program,
       ))


/*        .instruction(system_instruction::create_account(
            &authority,
            &wallet_address.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(Account::LEN).unwrap(),
            spl_token::state::Account::LEN as u64,
            &spl_token::id(),
        ))*/
/*        .instruction(spl_token::instruction::initialize_account(
            &token_program,
            &ata,
            &dummy_a.pubkey(),
            &authority,  //
        )
            .unwrap())*/
/*        .instruction( anchor_spl::token::MintTo(
            &token_program,
            &dummy_a.pubkey(),
            &ata,//&wallet_address.pubkey(),  //
            &authority,
            &[&authority],
            100,
        )
            .unwrap())*/

        .instruction( spl_token::instruction::mint_to(
            &token_program,
            &dummy_a.pubkey(),
            &ata,//&wallet_address.pubkey(),  //
            &authority,
            &[&authority],
            100,
        )
            .unwrap())
/*        .instruction(system_instruction::create_account(
            &program.payer(),
            &dummy_a.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(500).unwrap(),
            500,
            &program.id(),
        ))*/
     //  .signer(&wallet_address)
        .signer(&dummy_a)
        .signer(&counter)
        .accounts(basic_2_accounts::Initialize {
            my_account: counter.pubkey(),
         //   puppet_program: puppet_program_id,
          //  mint: mint_authority,
            user: authority,
            system_program: system_program::ID,
        })
        .args(basic_2_instruction::Initialize { data:20_u64,  mint_seed: string.to_vec(),mint_bump: mint_authority_bump_seed})

        .send().unwrap();


    let counter_account: my_solana_program::MyAccount = program.account(counter.pubkey()).unwrap();




  let ata_account: TokenAccount = program.account(ata).unwrap();
  //  assert_eq!(counter_account.authority, authority);
   // assert_eq!(counter_account.count, 0);
    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("Basic 2 success!{:?}",counter_account.data);
}

/*
fn main_为其他钱包建立token_account() {
    // RAY
    let usdc_mint_account_pubkey = Pubkey::from_str("FSRvxBNrQWX2Fy2qvKMLL3ryEdRtE3PUTZBcdKwASZTU").unwrap();
    //USDC
    //  let usdc_mint_account_pubkey = Pubkey::from_str("BEcGFQK1T1tSu3kvHC17cyCkQ5dvXqAJ7ExB2bb5Do7a").unwrap();

    let wallet_address = Pubkey::from_str("GnJpWUj3BmbH4KtaHhs52pnknVB3QqPLN68WxRfju2Sh").unwrap();

    let connection = RpcClient::new(URL);
    let mut payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    println!("frombyte_testKey private key= {:?}", payer.pubkey());



    println!("create_fund_receiper_spl_token_token_account");
    let lamport_requirement =
        connection.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();



    let ix = spl_associated_token_account::create_associated_token_account(
        &payer.pubkey(),
        &wallet_address,
        &usdc_mint_account_pubkey,
        //   token_program,
    );
    /*        invoke(
                &ix,
                &[payer, associated_token_account, wallet, token_mint, system_program, token_program, associated_token_account_program],
            )

       */
    let recent_blockhash = connection.get_latest_blockhash().unwrap();

    let create_new_token_account_tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    /*

            // create mint transaction
            let token_mint_a_tx = Transaction::new_signed_with_payer(
                &[initialize_account_ix],
                Some(&player.pubkey()),
                &[player, &mint_account],
                recent_blockhash,
            );
    */
    connection.send_and_confirm_transaction(&create_new_token_account_tx).unwrap();


}*/



fn basic_又contract端进行mint到给定(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.


    let string = b"seed";

    let (mint_authority, mint_authority_bump_seed) = Pubkey::find_program_address(&[string], &program.id());

    // `Initialize` parameters.
    let dummy_a = Keypair::generate(&mut OsRng);



    // Wallet and cluster params.

   let wallet_address = Keypair::generate(&mut OsRng);

    let authority = program.payer();





    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_address.pubkey(),
        &dummy_a.pubkey(),
        //   token_program,
    );


    //  let ata = Pubkey::from_str("8jtxPqpUV8oex35gNu5HkEjNySw4x64jLBYBmYYUft9y").unwrap();
    //   let dummy_a_key = Pubkey::from_str("EexCW6kpKiiQaK4fooG8UpfVxDHvAVxwPESyTvh1ocug").unwrap();


    //let ix = ;

    // Build and send a transaction.
    let token_program =spl_token::id();// &id;
    program
        .request()
        .instruction(anchor_client::solana_sdk::system_instruction::create_account(
            &program.payer(),
            &dummy_a.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap(),
            Mint::LEN  as u64,
            &anchor_spl::token::ID,
        ))
        .instruction(spl_token::instruction::initialize_mint(
            &token_program,
            &dummy_a.pubkey(),
            &mint_authority,  // 这里是mint 权限的人
            None,
            0,
        ).unwrap())

        .instruction(spl_associated_token_account::create_associated_token_account(
            &authority,
            &wallet_address.pubkey(),
            &dummy_a.pubkey(),
            //   token_program,
        ))


        .signer(&dummy_a)
        .accounts(basic_2_accounts::Airdrop {
            mint: dummy_a.pubkey(),
            destination:ata,
            mint_authority:mint_authority,
            payer: authority,
            token_program: spl_token::ID,
        })
        .args(basic_2_instruction::Airdrop {  mint_seed: string.to_vec(),mint_bump: mint_authority_bump_seed})

        .send().unwrap();


    //let counter_account: my_solana_program::MyAccount = program.account(dummy_a.pubkey()).unwrap();

    let counter_account: Mint = program.account(dummy_a.pubkey()).unwrap();

    let ata_account: TokenAccount = program.account(ata).unwrap();

    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("mint_account supply: {:?}",counter_account.supply);
}



fn basic_又contract端进行mint到给定_使用钱包公钥(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.


    let string = b"seed";

    let (mint_authority, mint_authority_bump_seed) = Pubkey::find_program_address(&[string], &program.id());

    // `Initialize` parameters.
    let dummy_a = Keypair::generate(&mut OsRng);



    // Wallet and cluster params.
    let wallet_address = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    //  let wallet_address = Keypair::generate(&mut OsRng);

    let authority = program.payer();





    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_address.pubkey(),
        &dummy_a.pubkey(),
        //   token_program,
    );


    //  let ata = Pubkey::from_str("8jtxPqpUV8oex35gNu5HkEjNySw4x64jLBYBmYYUft9y").unwrap();
    //   let dummy_a_key = Pubkey::from_str("EexCW6kpKiiQaK4fooG8UpfVxDHvAVxwPESyTvh1ocug").unwrap();


    //let ix = ;

    // Build and send a transaction.
    let token_program =spl_token::id();// &id;
    program
        .request()
        .instruction(anchor_client::solana_sdk::system_instruction::create_account(
            &program.payer(),
            &dummy_a.pubkey(),
            program.rpc().get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap(),
            Mint::LEN  as u64,
            &anchor_spl::token::ID,
        ))
        .instruction(spl_token::instruction::initialize_mint(
            &token_program,
            &dummy_a.pubkey(),
            &mint_authority,  // 这里是mint 权限的人
            None,
            0,
        ).unwrap())

        .instruction(spl_associated_token_account::create_associated_token_account(
            &authority,
            &wallet_address.pubkey(),
            &dummy_a.pubkey(),
            //   token_program,
        ))


        .signer(&dummy_a)
        .accounts(basic_2_accounts::Airdrop {
            mint: dummy_a.pubkey(),
            destination:ata,
            mint_authority:mint_authority,
            payer: authority,
            token_program: spl_token::ID,
        })
        .args(basic_2_instruction::Airdrop {  mint_seed: string.to_vec(),mint_bump: mint_authority_bump_seed})

        .send().unwrap();


    //let counter_account: my_solana_program::MyAccount = program.account(dummy_a.pubkey()).unwrap();

    let counter_account: Mint = program.account(dummy_a.pubkey()).unwrap();

    let ata_account: TokenAccount = program.account(ata).unwrap();

    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("mint_account supply: {:?}",counter_account.supply);
}




fn basic_又contract端进行mint到给定_使用钱包公钥_mint固定起来(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.


    let string = b"seed";

    let (mint_authority, mint_authority_bump_seed) = Pubkey::find_program_address(&[string], &program.id());

    // `Initialize` parameters.
  //  let dummy_a = Keypair::generate(&mut OsRng);



    // Wallet and cluster params.
    let wallet_address = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    //  let wallet_address = Keypair::generate(&mut OsRng);

    let authority = program.payer();



    let dummy_a_key = Pubkey::from_str("BV1fDj1T8pF1X3fdWZTGysmrWiuz1oaSrt3NvHCWA1Jk").unwrap();


    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_address.pubkey(),
        &dummy_a_key,
        //   token_program,
    );

    println!("ata address: {:?}",ata);




    //let ix = ;

    // Build and send a transaction.
    let token_program =spl_token::id();// &id;
    program
        .request()

/*
        .instruction(spl_associated_token_account::create_associated_token_account(
            &authority,
            &wallet_address.pubkey(),
            &dummy_a_key,
            //   token_program,
        ))

*/
       .signer(&wallet_address)
        .accounts(basic_2_accounts::Airdrop {
            mint: dummy_a_key,
            destination:ata,
            mint_authority:mint_authority,
            payer: authority,
            token_program: spl_token::ID,
        })
        .args(basic_2_instruction::Airdrop {  mint_seed: string.to_vec(),mint_bump: mint_authority_bump_seed})

        .send().unwrap();


    //let counter_account: my_solana_program::MyAccount = program.account(dummy_a.pubkey()).unwrap();

    let counter_account: Mint = program.account(dummy_a_key).unwrap();

    let ata_account: TokenAccount = program.account(ata).unwrap();

    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("mint_account supply: {:?}",counter_account.supply);
}






fn mint_nft_开始mint(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.


    let string = b"seed";

    let (mint_authority, mint_authority_bump_seed) = Pubkey::find_program_address(&[string], &program.id());

    // `Initialize` parameters.
    //  let dummy_a = Keypair::generate(&mut OsRng);



    // Wallet and cluster params.
    let wallet_address = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    //  let wallet_address = Keypair::generate(&mut OsRng);

    let authority = program.payer();



    let dummy_a_key = Pubkey::from_str("BV1fDj1T8pF1X3fdWZTGysmrWiuz1oaSrt3NvHCWA1Jk").unwrap();


    let ata = spl_associated_token_account::get_associated_token_address(
        &wallet_address.pubkey(),
        &dummy_a_key,
        //   token_program,
    );

    println!("ata address: {:?}",ata);




    //let ix = ;

    // Build and send a transaction.
    let token_program =spl_token::id();// &id;
    program
        .request()

        /*
                .instruction(spl_associated_token_account::create_associated_token_account(
                    &authority,
                    &wallet_address.pubkey(),
                    &dummy_a_key,
                    //   token_program,
                ))

        */

        .signer(&wallet_address)
        .accounts(basic_2_accounts::MintNft {
            metadata: dummy_a_key,
            mint:ata,
            mint_authority:mint_authority,
            payer: authority,
            token_metadata_program: spl_token::ID,
            token_program: spl_token::ID,
            system_program: spl_token::ID,
            rent: spl_token::ID,

        })
        .args(basic_2_instruction::Airdrop {  mint_seed: string.to_vec(),mint_bump: mint_authority_bump_seed})

        .send().unwrap();


    //let counter_account: my_solana_program::MyAccount = program.account(dummy_a.pubkey()).unwrap();

    let counter_account: Mint = program.account(dummy_a_key).unwrap();

    let ata_account: TokenAccount = program.account(ata).unwrap();

    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("mint_account supply: {:?}",counter_account.supply);
}




/*fn basic_建立支付保险箱(client: &Client, pid: Pubkey)  {
    let program = client.program(pid);

    // `Create` parameters.


    let offer = Keypair::generate(&mut OsRng);

    let (escrowedTokensOfOfferMaker, escrowedTokensOfOfferMakerBump) = Pubkey::find_program_address(
       // [offer.pubkey().toBuffer()],
       &[&offer.pubkey().to_bytes()],

        program.programId
    );



/*    const [bedrijfPDA, bump] = await PublicKey.findProgramAddress(
        [
            anchor.utils.bytes.utf8.encode("bedrijfpda"), // hiervoor hebben we deze seed nodig
            Uint8Array.of(id),
            wallet.publicKey.toBytes(),                   // en de publicKey van de wallet
        ],
        program.programId // we leiden af vanaf ons huidige programma
    );
    */




    let authority = program.payer();

    let dummy_a_key = Pubkey::from_str("BV1fDj1T8pF1X3fdWZTGysmrWiuz1oaSrt3NvHCWA1Jk").unwrap();



    println!("ata address: {:?}",ata);



    let token_program =spl_token::id();// &id;
    program
        .request()

        .signer(&wallet_address)
        .accounts(basic_2_accounts:: {
            offer: offer.publicKey,
            whoMadeTheOffer: program.provider.wallet.publicKey,
            tokenAccountFromWhoMadeTheOffer: offerMakerCowTokenAccount,
            escrowedTokensOfOfferMaker: escrowedTokensOfOfferMaker,
            kindOfTokenOffered: cowMint.publicKey,
            kindOfTokenWantedInReturn: pigMint.publicKey,
            tokenProgram: spl.TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .args(basic_2_instruction::Airdrop {  mint_seed: escrowedTokensOfOfferMakerBump,
            mint_bump: mint_authority_bump_seed})

        .send().unwrap();


/*    await program.rpc.makeOffer(
        escrowedTokensOfOfferMakerBump,
        new anchor.BN(2),
        new anchor.BN(4),
        {
            accounts: ,
            signers: [offer]
        }
    );*/
    let counter_account: Mint = program.account(dummy_a_key).unwrap();

    let ata_account: TokenAccount = program.account(ata).unwrap();

    println!("mint_account 2 mint !{:?}",ata_account.mint);
    println!("mint_account key !{:?}",ata);

    println!("mint_account supply: {:?}",counter_account.supply);
}*/

