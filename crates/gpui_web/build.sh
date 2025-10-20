#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if wasm-pack is installed
check_wasm_pack() {
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack is not installed!"
        echo "Install it with: cargo install wasm-pack"
        exit 1
    fi
}

# Build the WASM module
build() {
    print_info "Building GPUI Web WASM module..."

    local target=${1:-web}
    local profile=${2:-dev}

    if [ "$profile" = "release" ]; then
        wasm-pack build --target $target --release
    else
        wasm-pack build --target $target --dev
    fi

    print_info "Build complete! Output in pkg/"
}

# Run tests
test() {
    print_info "Running tests in browser..."

    local browser=${1:-firefox}

    wasm-pack test --headless --$browser

    print_info "Tests complete!"
}

# Run tests with console output
test_debug() {
    print_info "Running tests with debug output..."

    RUST_LOG=debug wasm-pack test --firefox
}

# Serve the example locally
serve() {
    print_info "Building and serving example..."

    # Build in release mode for the example
    build web release

    # Create a simple HTML file if it doesn't exist
    if [ ! -f "example/index.html" ]; then
        mkdir -p example
        cat > example/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>GPUI Web Example</title>
    <style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            margin: 40px;
            background: #1e1e1e;
            color: #d4d4d4;
        }
        h1 { color: #4fc3f7; }
        #output {
            background: #2d2d2d;
            border: 1px solid #3e3e3e;
            border-radius: 4px;
            padding: 20px;
            margin-top: 20px;
            min-height: 200px;
            white-space: pre-wrap;
            font-family: 'Monaco', 'Menlo', monospace;
        }
    </style>
</head>
<body>
    <h1>GPUI Web Platform Example</h1>
    <div id="output">Loading WASM module...</div>

    <script type="module">
        import init, { GpuiWeb } from '../pkg/gpui_web.js';

        async function run() {
            const output = document.getElementById('output');

            try {
                // Initialize the WASM module
                await init();
                output.textContent = 'WASM module loaded!\n\n';

                // Create platform instance
                const platform = new GpuiWeb();

                // Get hardware info
                const cores = platform.getHardwareConcurrency();
                output.textContent += `Hardware concurrency: ${cores} cores\n`;

                // Log a message
                platform.log('Hello from GPUI Web!');
                output.textContent += 'Logged message to console\n';

                // Spawn some async tasks
                for (let i = 0; i < 3; i++) {
                    platform.spawnTask(async () => {
                        const delay = Math.random() * 1000;
                        await new Promise(resolve => setTimeout(resolve, delay));
                        const msg = `Task ${i} completed after ${delay.toFixed(0)}ms`;
                        output.textContent += msg + '\n';
                        console.log(msg);
                    });
                }

                output.textContent += '\nSpawned 3 async tasks (check console for output)\n';

            } catch (error) {
                output.textContent = `Error: ${error}\n`;
                console.error(error);
            }
        }

        run();
    </script>
</body>
</html>
EOF
        print_info "Created example/index.html"
    fi

    # Check if Python 3 is available
    if command -v python3 &> /dev/null; then
        print_info "Starting web server at http://localhost:8000"
        cd example && python3 -m http.server 8000
    else
        print_error "Python 3 not found. Please install Python 3 or use another web server."
        print_info "Example files are in the 'example' directory"
    fi
}

# Clean build artifacts
clean() {
    print_info "Cleaning build artifacts..."

    rm -rf pkg/
    rm -rf target/wasm32-unknown-unknown/
    cargo clean

    print_info "Clean complete!"
}

# Show usage
usage() {
    echo "GPUI Web Build Script"
    echo ""
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  build [target] [profile]  Build the WASM module (default: web, dev)"
    echo "  test [browser]            Run tests (default: firefox)"
    echo "  test-debug                Run tests with debug output"
    echo "  serve                     Build and serve example"
    echo "  clean                     Clean build artifacts"
    echo "  help                      Show this help message"
    echo ""
    echo "Targets:"
    echo "  web          For use with ES modules (default)"
    echo "  bundler      For use with bundlers like Webpack"
    echo "  nodejs       For use in Node.js"
    echo ""
    echo "Profiles:"
    echo "  dev          Development build (default)"
    echo "  release      Optimized release build"
    echo ""
    echo "Browsers:"
    echo "  firefox      Firefox (default)"
    echo "  chrome       Chrome/Chromium"
    echo "  safari       Safari"
}

# Main script logic
main() {
    check_wasm_pack

    case ${1:-build} in
        build)
            build ${2:-web} ${3:-dev}
            ;;
        test)
            test ${2:-firefox}
            ;;
        test-debug)
            test_debug
            ;;
        serve)
            serve
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            usage
            ;;
        *)
            print_error "Unknown command: $1"
            usage
            exit 1
            ;;
    esac
}

main "$@"
