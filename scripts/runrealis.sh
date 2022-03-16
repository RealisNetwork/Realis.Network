#!/bin/bash

#Intro
clear

echo -e "                                         \e[38;5;208m|\__/|
                                        /     \\
                                       /_.\e[39m~ ~\e[38;5;208m,_\   
                                          \@/ \e[39m"
echo -e ""
echo -e "                           \e[38;5;208mRealis.Network Validator \e[38;5;82mfull installer \e[39m"
sleep 1s 
echo -e "                           \e[38;5;82mBy \e[38;5;196mRasmonT \e[39m"
sleep 2s
echo -e "                           \e[38;5;82mChecking if user is \e[38;5;9mroot\e[38;5;82m... \e[39m"
sleep 1s

#Check user
if [ $USER == 'root' ]
then
echo -e "                           \e[38;5;9mPlease create a new user with admin privileges!\e[39m"
echo -e "                           \e[38;5;9mGuide: \e[38;5;11mhttps://www.digitalocean.com/community/tutorials/how-to-create-a-new-sudo-enabled-user-on-ubuntu-20-04-quickstart\e[39m"
echo -e "                           \e[38;5;9mFor safety reason, validator shouldn't be ran as a root.\e[39m"
echo -e "                           \e[38;5;82mBut installation will continue as a root user \e[39m"
sleep 1s
echo -e "\e[38;5;12mTelegram: \e[38;5;11m@RasmonT \e[38;5;14mDiscord: \e[38;5;11mRasmonT#9018"
echo -e "\e[38;5;12mTelegram Chat: \e[38;5;11m@RealisENG \e[38;5;14mDiscord: \e[38;5;11mhttps://discord.gg/YRjpPW2jz4"
echo -e "\e[38;5;82mIf you have any questions, or need assistance, please message me! \e[39m"
sleep 1s
echo -e "                           \e[38;5;82mProceeding... \e[39m"
else
echo -e "                           \e[38;5;82mProceeding... \e[39m"
fi

#Install upgrades
echo -e "                           \e[38;5;82mChecking for updates. \e[39m"
sudo apt-get update && upgrade
sleep 2s

sleep 1s
echo -e "                           \e[38;5;82mCreating Service file... \e[39m"
sleep 1s

#Validator name
echo -e "                           \e[38;5;82mEnter your Validator name \e[39m"
read varname
echo -e "                           \e[38;5;82mYour Validator name is \e[38;5;229m$varname \e[39m" 

#Creating Service file
sudo tee /etc/systemd/system/validator.service > /dev/null <<EOF
# /etc/systemd/system/validator.service
[Unit]
Description=Realis Validator
After=network-online.target

[Service]
User=$USER
WorkingDirectory=$HOME/Realis.Network
ExecStart=$HOME/Realis.Network/target/release/realis --chain ./realis.json --port 30334 --ws-port 9945 --rpc-port 9934 --validator --rpc-methods=Unsafe --name $varname --reserved-nodes /ip4/135.181.18.215/tcp/30333/p2p/12D3KooW9poizzemF6kb6iSbkoJynMhswa4oJe5W9v34eFuRcU47 --unsafe-ws-external --unsafe-rpc-external --rpc-cors '*' -d ../realis/node --telemetry-url 'wss://telemetry.polkadot.io/submit 0'
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target

EOF
sleep 1s
echo -e "                           \e[38;5;82mFile created! Continuing with installation... \e[39m"
sleep 1s

#Installing Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

sleep 2s
if [[ "$(rustc --version)" ==  "rustc"* ]]; then
echo -e "                           \e[38;5;82mInstallation of Rust susscesful! \e[39m"
else
echo -e "                           \e[38;5;9mInstallation of Rust failed! \e[39m"
exit 1
fi

sleep 1s
echo -e "                           \e[38;5;82mNow I will install importnant dependencies... \e[39m"

#Instaliing dependencies + other things
sudo apt install make clang pkg-config libssl-dev build-essential
sleep 2s
sudo apt install systemd-timesyncd 
timedatectl set-ntp true
sudo apt install htop
sudo apt install ccze
sudo apt install git
echo -e "                           \e[38;5;208mDownloading Realis... \e[39m"
sleep 2s

#Downloading Realis
git clone https://github.com/RealisNetwork/Realis.Network.git
sleep 1s
cd Realis.Network
# git tag -l | sort -V | grep -v -- '-rc'
git checkout prod
sleep 1s
rustup install nightly-2021-08-30
rustup target add wasm32-unknown-unknown --toolchain nightly-2021-08-30
cargo build --release
echo -e "                           \e[38;5;82mStarting validator... \e[39m"
sleep 2s

#Enabling Realis 
sudo systemctl start validator
sleep 2s
sudo systemctl --no-pager status validator
sleep 5s
trap exit INT

#Generate Keys
echo "Do you want me to generate session keys?[y/n]"
read DOSETUP

if [[ $DOSETUP =~ "n" ]] ; then
echo -e "                           \e[38;5;11mCanceling session key generation... \e[39m"
rm -rf $HOME/runrealis.sh
sleep 2s      
echo -e "\e[38;5;82mPlease use curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933 to generate keys \e[39m"
sleep 1s
echo -e "\e[38;5;12mTelegram: \e[38;5;11m@RasmonT \e[38;5;14mDiscord: \e[38;5;11mRasmonT#9018"
echo -e "\e[38;5;12mTelegram Chat: \e[38;5;11m@RealisENG \e[38;5;14mDiscord: \e[38;5;11mhttps://discord.gg/YRjpPW2jz4"
echo -e "\e[38;5;82mIf you have any questions, or need assistance, please message me!"
sleep 2s
fi

if [[ $DOSETUP =~ "y" ]] ; then
echo -e "                           \e[38;5;82mGenerating keys... \e[39m"
rm -rf $HOME/runrealis.sh
sleep 3s
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9933
sleep 1s
echo -e "                           \e[38;5;82mKeys Generated... \e[39m"
echo -e "\e[38;5;12mTelegram: \e[38;5;11m@RasmonT \e[38;5;14mDiscord: \e[38;5;11mRasmonT#9018"
echo -e "\e[38;5;12mTelegram Chat: \e[38;5;11m@RealisENG \e[38;5;14mDiscord: \e[38;5;11mhttps://discord.gg/YRjpPW2jz4"
echo -e "\e[38;5;82mIf you have any questions, or need assistance, please message me! \e[39m"
sleep 2s
fi

#Finish 
echo -e "\e[38;5;11mCommands:"
echo -e "\e[38;5;10msudo systemctl status validator \e[38;5;11m- Check validator status"
echo -e "\e[38;5;10msudo systemctl stop validator \e[38;5;9m- Stop validator"
echo -e "\e[38;5;10msudo systemctl start validator \e[38;5;76m- Start validator"
echo -e "------------------------------------------------------------------------------------"
echo -e "\e[38;5;82mUse this command to see your validator processing blocks."
echo -e "\e[38;5;45mjournalctl -u validator -f -q  | ccze -A -o nolookups"
echo -e "\e[38;5;11mYou can leave block session anytime by pressing \e[38;5;1mctrl-c \e[39m"
sleep 1s
trap ctrl_c INT
exit 1




