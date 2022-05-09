FROM ethereum/client-go:stable

COPY ./docker/genesis.json /tmp/genesis.json
COPY ./docker/init.sh /script/init.sh
RUN chmod +x /script/init.sh

ENTRYPOINT ["/script/init.sh"] 