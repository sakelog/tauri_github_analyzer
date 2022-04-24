import { utcToZonedTime, format } from 'date-fns-tz';

export const formatDate = (targetString: string) => {
  const objDate = new Date(targetString);
  const TIME_ZONE = 'Asia/Tokyo';
  const zonedDate = utcToZonedTime(objDate, TIME_ZONE);

  const FORMAT_PATTERN = 'yyyy-MM-dd';
  return format(zonedDate, FORMAT_PATTERN, {
    timeZone: TIME_ZONE,
  });
};
