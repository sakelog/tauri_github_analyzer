export const sortInfoList = (
  targetArray: Array<Github.RepoInfo>,
  mode: 'views' | 'clones'
) => {
  const sortByCount = (
    a: Github.RepoInfo,
    b: Github.RepoInfo
  ) => {
    const countA =
      mode === 'views'
        ? a.views_info.count
        : a.clones_info.count;
    const countB =
      mode === 'views'
        ? b.views_info.count
        : b.clones_info.count;

    if (countA > countB) {
      return -1;
    }
    if (countA < countB) {
      return 1;
    }

    return 0;
  };

  targetArray.sort(sortByCount);

  return targetArray;
};

export default { sortInfoList };
