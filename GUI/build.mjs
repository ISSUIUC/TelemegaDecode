import * as esbuild from 'esbuild'

let ctx = await esbuild.context({
    entryPoints: ['./frontend_src/main.ts'],
    outfile: 'public/bundle.js',
    sourcemap: true,
    target: "chrome120",
    bundle: true,
    minify: false,
    loader: {'.wgsl': 'text'},
});

await ctx.watch();
console.log("waiting");