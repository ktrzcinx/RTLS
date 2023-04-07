# Webpack content

This frontend uses `webpack` to serve http node server.
`webpack` tutorial: (webpack.js.org/guides/getting-started/)[with https://webpack.js.org/guides/getting-started/].
IIn this project, `src` is a folder with developer editable content, mainly
`.js` files and `icons`. After invocation of `npn run build` (which is an alias
to `npx webpack --config webpack.config.js --mode=development`) `webpack` will
translate every needed source file and create `dist` folder with `index.html`,
needed `.js` scripts and another resources.

For developing purposes, it is convenient to bundle files on the fly.
To to this just run:

```text
npm install
npm run build
npm run start
```

In this mode, after every source file save the effect should be visible in
the web browser.
