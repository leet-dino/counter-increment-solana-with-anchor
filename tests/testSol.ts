import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TestSol } from '../target/types/test_sol';

describe('testSol', () => {

  // // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.Provider.env());

  // const program = anchor.workspace.TestSol as Program<TestSol>;

  // it('Is initialized!', async () => {
  //   // Add your test here.
  //   const tx = await program.rpc.initialize({});
  //   console.log("Your transaction signature", tx);
  // });

  const provider = anchor.Provider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.TestSol as Program<TestSol>;
  const counter = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    await program.rpc.create({
      accounts: {
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [counter]
    } as any)
  });

  it("Increment counter", async () => {
    await program.rpc.increment({
      accounts: {
        counter: counter.publicKey
      }
    } as any)
  })

  it("Fetch account", async () => {
    const account: any = await program.account.counter.fetch(counter.publicKey)
    console.log(account.count)
  })



});
