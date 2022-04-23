declare namespace GitHub {
  export type Views = {
    timestamp: string;
    uniques: number;
    count: number;
  };
  export type TrafficResult = {
    name: string;
    count: number;
    views: Array<Views>;
  };
}
