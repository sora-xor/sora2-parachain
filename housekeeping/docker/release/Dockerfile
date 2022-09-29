FROM debian:buster-slim
ENV POLKADOT_API_PATH=/usr/local/lib/node_modules/@polkadot
# Install dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
        npm curl make cmake && \
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete
RUN npm install -g @polkadot/api-cli@0.33.1

RUN useradd substrate -u 10000

COPY ./housekeeping/parachain-collator /usr/local/bin/parachain-collator
COPY --chown=10000:10000 ./housekeeping/docker/release/parachain_registration.sh /opt/sora2/parachain_registration.sh
RUN chmod +x /opt/sora2/parachain_registration.sh  && \
    mkdir /chain && \
    chown 10000:10000 /chain

USER substrate

ENTRYPOINT ["parachain-collator"]