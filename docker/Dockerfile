FROM ubuntu
#ADD /blockchain_soul/ReAlis-Network/target/release/realis /realis/realis0
#ENTRYPOINT ["bash","entrypoint.prod.sh"]
ADD ./realis /realis/realis
RUN chmod +x /realis/realis
ADD ./realis.json /realis/realis.json
WORKDIR /realis/chain
EXPOSE 9944 9044 9033
