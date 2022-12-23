import { Typhoon } from "./definition";

let data: Typhoon[];
let typhoonIndex: { [key: string]: Typhoon } = {};

export const init = (d: Typhoon[]) => {
  data = d;
  for (let t of d) {
    const { header } = t;
    typhoonIndex[header.InternationalNumberID] = t;
  }
};

export const getRaw = () => {
  return data;
};

const dPattern = /(\d{2})(\d{2})(\d{2})(\d{2})/;
const parseTwoDigitYear = (yearNum: number) => {
  return yearNum + (yearNum > 22 ? 1900 : 2000);
};

export const dateParse = (dateStr: string) => {
  const [yy, mm, dd, hh] = dateStr
    .split(dPattern)
    .filter((v) => v)
    .map((v) => parseInt(v));
  return new Date(parseTwoDigitYear(yy), mm - 1, dd, hh);
};

export const getHeaders = () => {
  return data
    .map((t) => {
      return {
        ...t.header,
        last: dateParse(t.header.Last),
      };
    })
    .sort((a, b) => {
      return b.last.getTime() - a.last.getTime();
    });
};

export const getTyphoon = (id: string) => {
  return typhoonIndex[id];
};
