#!bin/sh

rm -f ~/.ethereum/geth/nodekey
echo ${ACCOUNT_PASSWORD} > /tmp/password
geth --datadir=${PRIVATECHAIN_PATH} account new --password /tmp/password
rm -f /tmp/password
geth --datadir=${PRIVATECHAIN_PATH} init /tmp/genesis.json
geth --datadir=${PRIVATECHAIN_PATH} \
      --nodekeyhex="b0ac22adcad37213c7c565810a50f1772291e7b0ce53fb73e7ec2a3c75bc13b5" \
      --nodiscover \
      --networkid=${NETWORK_ID} \
      --allow-insecure-unlock \
      --http \
      --http.addr="0.0.0.0" \
      --http.api="eth,web3,net,admin,personal" \
      --http.corsdomain="*" \
      --mine \
      --miner.threads=1 \
