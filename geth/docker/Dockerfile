FROM ethereum/client-go:stable
ARG ACCOUNT_PASSWORD
ARG PRIVATECHAIN_PATH
COPY ./docker/genesis.json /tmp/genesis.json

RUN rm -f ~/.ethereum/geth/nodekey \
    && echo ${ACCOUNT_PASSWORD} > /tmp/password \
    && geth --datadir=${PRIVATECHAIN_PATH} account new --password /tmp/password \
    && rm -f /tmp/password
RUN geth --datadir=${PRIVATECHAIN_PATH} init /tmp/genesis.json

ENTRYPOINT ["geth"]