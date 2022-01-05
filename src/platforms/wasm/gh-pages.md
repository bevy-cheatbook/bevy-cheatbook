# Hosting on GitHub Pages

GitHub Pages is a hosting service that allows you to publish your website
on GitHub's servers.

For more details, visit the official [GitHub Pages
documentation](https://docs.github.com/en/pages).

Deploying a website (like your WASM game) to GitHub pages is done by
putting the files in a special branch in a GitHub repository. You could
create a separate repository for this, but you could also do it from the
same repository as your source code.

You will need the final website files for deployment. You are probably
generating these using a tool like `wasm-bindgen` or `wasm-pack`, maybe
as part of a larger website.

Create an empty branch in your git repository:

```shell
git checkout --orphan web
git reset --hard
```

You should now be in an empty working directory.

Put all files necessary for hosting, including your HTML, WASM, JavaScript,
and `assets` files, and commit them into git:

```shell
git add *
git commit
```

(or better, manually list your files in the above command, in place of the `*` wildcard)

Push your new branch to GitHub:

```shell
git push -u origin web --force
```

In the GitHub Web UI, go to the repository settings, go to the "GitHub Pages"
section, then under "Source" pick the branch "web" and the `/` (root) folder.
Then click "Save".

Wait a little bit, and your site should become available at
`https://your-name.github.io/your-repo`.
