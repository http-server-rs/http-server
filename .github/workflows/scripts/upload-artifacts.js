const fs = require('fs').promises;
const { ARTIFACT_NAME, ASSET_NAME, GITHUB_REF } = process.env;

module.exports = async ({ github, context }) => {
  const {
    repo: {
      owner,
      repo,
    },
  } = context;
  const tag = GITHUB_REF.replace('refs/tags/', '');

  const release = await github.repos.getReleaseByTag({
    owner,
    repo,
    tag,
  });

  const files = await fs.readdir('./target/release');

  for (let file of files) {
    if (file === ASSET_NAME) {
      await github.repos.uploadReleaseAsset({
        owner,
        repo,
        release_id: release.data.id,
        name: ARTIFACT_NAME,
        data: await fs.readFile(file),
      });
    }
  }
}
