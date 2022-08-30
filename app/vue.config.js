const webpack = require('webpack')
const { defineConfig } = require('@vue/cli-service')

module.exports = defineConfig({
    transpileDependencies: true,
    configureWebpack: {
        plugins: [
            new webpack.ProvidePlugin({
                Buffer: ['buffer', 'Buffer']
            })
        ],
        resolve: {
            fallback: {
                crypto: false,
                fs: false,
                assert: false,
                process: false,
                util: false,
                path: false,
                stream: false,
            }
        }
    }
})


// In the early days, webpack's aim was to allow running most Node.js modules in the browser, 
// but the module landscape changed and many module uses are now written mainly for frontend purposes. 
// Webpack <= 4 ships with polyfills for many of the Node.js core modules, which are automatically applied 
// once a module uses any of the core modules (i.e. the crypto module).

// Webpack 5 stops automatically polyfilling these core modules and focus on frontend-compatible modules. 
// Our goal is to improve compatibility with the web platform, where Node.js core modules are not available.