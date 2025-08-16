import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KindNet } from "../target/types/kind_net";
import { assert } from "chai";

describe("kind_net", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.KindNet as Program<KindNet>;

  it("Menyimpan alamat pengguna dengan sukses!", async () => {
    const user = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 1000000000), // 1 SOL
      "confirmed"
    );

    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_address"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .saveAddrs()
      .accounts({
        addressData: pda,
        signer: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const storedData = await program.account.addressData.fetch(pda);

    assert.equal(
      storedData.owner.toString(),
      user.publicKey.toString(),
      "Pemilik brankas tidak sesuai!"
    );

    console.log("====================================");
    console.log("🚀 Success!");
    console.log("📭 PDA ID:", pda.toString());
    console.log("👤 Owner:", storedData.owner.toString());
    console.log(
      "💾 Storage Size:",
      await provider.connection
        .getAccountInfo(pda)
        .then((info) => info.data.length),
      "bytes"
    );
    console.log("====================================");
  });
});
