export const sortInfoList = (
  targetArray: Array<Github.RepoInfo>,
  mode: 'views' | 'clones'
) => {
  targetArray.sort((a, b) =>
    mode === 'views'
      ? b.views_info.count - a.views_info.count
      : b.clones_info.count - a.clones_info.count
  );

  return targetArray;
};

export default { sortInfoList };
