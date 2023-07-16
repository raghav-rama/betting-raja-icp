FROM node:18
WORKDIR /app
# RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
RUN DFX_VERSION=0.11.1 sh -ci "$(curl -sSL https://internetcomputer.org/install.sh)"
RUN ~/.cache/dfinity/uninstall.sh && sh -ci "$(curl -sSL https://internetcomputer.org/install.sh)"
# RUN echo '{ "local": { "bind": "127.0.0.1:8080", "type": "ephemeral", "replica": { "subnet_type": "system" } } }' > ~/.config/dfx/networks.json
COPY . .
RUN npm install
# RUN dfx start --background --clean
# RUN dfx deploy
# CMD ["npm", "start"]
