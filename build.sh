# Build process
# cf. https://betterprogramming.pub/deploying-a-wasm-powered-react-app-on-vercel-cf3cae2a75d6

# Install Rustup (compiler)
echo "Installing Rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Adding binaries to path
echo "Home directory: $HOME"
source "$HOME/.cargo/env"

# Install wasm-pack
echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

# Build wasm-parser 
echo "Building wasm-parser..."
npm run build:wasm

# Build static html for the client
echo "Build static frontend client..."
npm run build:web