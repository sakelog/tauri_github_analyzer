import { Octokit } from '@octokit/core';

export const githubApiClient = new Octokit({
  auth: process.env.REACT_APP_GITHUB_PERSONAL_ACCESS_TOKEN,
});

export default { githubApiClient };
