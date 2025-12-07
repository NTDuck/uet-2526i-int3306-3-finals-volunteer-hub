import { defineNuxtConfig } from 'nuxt/config'
import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
    compatibilityDate: '2025-11-26',
    css: ['./app/assets/css/main.css'],
    vite: {
        plugins: [tailwindcss()],
        assetsInclude: ['**/*.wasm']
    },

    app: {
        head: {
            title: 'VolunteerHub',
            htmlAttrs: {
                lang: 'en',
            },
            link: [
                { rel: 'icon', type: 'image/x-icon', href: '/favicon.png' },
            ],
        },
    },

    typescript: {
        typeCheck: true,
        strict: true,

        // customize tsconfig.app.json
        tsConfig: {
            // ...
        },
        // customize tsconfig.shared.json
        sharedTsConfig: {
            // ...
        },
        // customize tsconfig.node.json
        nodeTsConfig: {
            // ...
        }
    },

    nitro: {
        typescript: {
            // customize tsconfig.server.json
            tsConfig: {
                // ...
            }
        },
        experimental: {
            wasm: true
        },
        externals: {
            inline: ['@local/volunteer-hub']
        },
        esbuild: {
            options: {
                target: 'esnext'
            }
        }
    },

    devtools: {
        enabled: false
    },

    devServer: {
        port: 5000
    },

    modules: ['@nuxt/eslint']
})
