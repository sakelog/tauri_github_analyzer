import { githubApiClient } from 'lib/github/client';

export const getTraffic = async (): Promise<Array<GitHub.TrafficResult>> => {
  const { data: resUser } = await githubApiClient.request('GET /user', {});
  const { login } = resUser;

  const { data: resRepos } = await githubApiClient.request(
    `GET /user/repos`,
    {}
  );

  const resultArray = await Promise.all(
    resRepos.map(async (res) => {
      const { data } = await githubApiClient.request(
        'GET /repos/{owner}/{repo}/traffic/views',
        {
          owner: login,
          repo: res.name,
        }
      );
      return {
        name: res.name,
        count: data.count,
        views: data.views,
      };
    })
  );

  return resultArray;
};
