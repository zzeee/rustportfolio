const port = process.env.HOST_PORT || 9090

module.exports = {
  networks: {
    mainnet: {
      address:'TMJzeBma9zZrG95n5rNN7w3qLo4wCyrBrW',
      // Don't put your private key here:
      privateKey: process.env.PRIVATE_KEY_MAINNET || '07b1f080f3c674fcf7b284cd4a3ca4a232e213b78d676fd4d0a242d8eecbf3fe',
      /*
Create a .env file (it must be gitignored) containing something like
export PRIVATE_KEY_MAINNET=4E7FECCB71207B867C495B51A9758B104B1D4422088A87F4978BE64636656243
Then, run the migration with: source .env && tronbox migrate --network mainnet
*/    userFeePercentage: 100,
      feeLimit: 1e8,
      fullHost: 'https://api.trongrid.io',
      network_id: '1'
    },
    shasta: {
      privateKey: 'd506ff20115503c4a55fab395d638025cf8a2475ba739e08f4367838b86e7a1d' || process.env.PRIVATE_KEY_SHASTA,
      userFeePercentage: 50,
      feeLimit: 1e8,
      fullHost: 'https://api.shasta.trongrid.io',
      network_id: '2'
    },
    nile: {
      privateKey: process.env.PRIVATE_KEY_NILE,
      userFeePercentage: 100,
      feeLimit: 1e8,
      fullHost: 'https://api.nileex.io',
      network_id: '3'
    },
    development: {
      // For trontools/quickstart docker image
      privateKey: 'da146374a75310b9666e834ee4ad0866d6f4035967bfc76217c5a495fff9f0d0',
      userFeePercentage: 0,
      feeLimit: 1e8,
      fullHost: 'http://127.0.0.1:' + port,
      network_id: '9'
    },
    compilers: {
      solc: {
        version: '0.5.15'
      }
    }
  }
}
