declare namespace GitHub {
  export type RepoInfo = {
    name: string;
    url: string;
    views_info: TrafficInfo;
    clones_info: TrafficInfo;
  };

  export type Traffic = {
    timestamp: string;
    uniques: number;
    count: number;
  };
  export interface TrafficInfo {
    count: number;
    uniques: number;
    items: Array<Traffic>;
  }
}
