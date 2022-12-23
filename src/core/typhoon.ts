import {
  CallbackProperty,
  Cartesian3,
  Color,
  CustomDataSource,
  DataSource,
  EllipseGraphics,
  Entity,
  JulianDate,
  MaterialProperty,
  PointGraphics,
  SampledPositionProperty,
  SampledProperty,
  TimeInterval,
  TimeIntervalCollection,
  Viewer,
} from "cesium";
import { dateParse } from "./dataState";
import { Typhoon, Point, Header } from "./definition";
import { LineFlowMaterialProperty } from "./path";

let WIDGET: Viewer;
let SOURCE: DataSource;

class TyphoonPathEntity extends Entity {
  constructor(typhoon: Typhoon) {
    const { points } = typhoon;
    const { times, positions } = points.reduce<{
      times: JulianDate[];
      positions: Cartesian3[];
    }>(
      (acc, _point) => {
        const { times, positions } = acc;
        const { Time, Longitude, Latitude } = _point;
        try {
          times.push(JulianDate.fromDate(dateParse(Time)));
        } catch (e) {
          console.log(typhoon.header);
        }
        positions.push(Cartesian3.fromDegrees(Longitude, Latitude));
        return acc;
      },
      { times: [], positions: [] }
    );
    const { start, stop } = times.reduce<{
      start: JulianDate;
      stop: JulianDate;
    }>(
      (acc, time) => {
        const { start, stop } = acc;
        acc.start = JulianDate.lessThan(time, start) ? time.clone() : start;
        acc.stop = JulianDate.greaterThan(time, stop) ? time.clone() : stop;
        return acc;
      },
      { start: times[0].clone(), stop: times.slice(-1).pop()!.clone() }
    );
    const availability = new TimeIntervalCollection([
      new TimeInterval({
        start,
        stop,
      }),
    ]);
    const position = new SampledPositionProperty();
    position.addSamples(times, positions);
    const point = new PointGraphics({
      pixelSize: 5,
    });
    const option: Entity.ConstructorOptions = {
      position,
      availability,
      point,
      path: {
        resolution: 3600,
        material: new LineFlowMaterialProperty({
          color: Color.CYAN,
          percent: 0.5,
          speed: 25,
          gradient: 0.1,
        }),
      },
    };
    super(option);
  }
}

interface ITyphoonCircleEntity {
  typhoon: Typhoon;
  parent: Entity;
  axisType: keyof Point;
  material: MaterialProperty | Color | undefined;
  zIndex: number;
}
class TyphoonCircleEntity extends Entity {
  constructor({ typhoon, parent, axisType, material, zIndex }: ITyphoonCircleEntity) {
    const { points } = typhoon;
    const { position, availability, path } = parent;
    const { times, axies } = points.reduce<{
      times: JulianDate[];
      axies: Number[];
    }>(
      (acc, point) => {
        const { times, axies } = acc;
        const { Time } = point;
        times.push(JulianDate.fromDate(dateParse(Time)));
        const axis = (point[axisType] as number) || 0;
        axies.push(axis);
        return acc;
      },
      { times: [], axies: [] }
    );
    const axisSample = new SampledProperty(Number);
    axisSample.addSamples(times, axies);
    const axisValue = () =>
      new CallbackProperty((time) => {
        const _axis = axisSample.getValue(time);
        if (!_axis) {
          return 10.0;
        } else {
          return _axis * 1852.0;
        }
      }, false);
    const ellipse = new EllipseGraphics({
      semiMajorAxis: axisValue(),
      semiMinorAxis: axisValue(),
      material,
      zIndex,
    });
    const option: Entity.ConstructorOptions = {
      position,
      availability,
      parent,
      path,
      ellipse,
    };
    super(option);
  }
}

const addTyphoon = (typhoonDatas: Typhoon[]) => {
  typhoonDatas.map((typhoon) => {
    const parent = SOURCE.entities.add(new TyphoonPathEntity(typhoon));
    SOURCE.entities.add(
      new TyphoonCircleEntity({
        typhoon,
        parent,
        axisType: "LongestRadius30",
        material: Color.fromCssColorString("#0070FF50"),
        zIndex: 1,
      })
    );
    SOURCE.entities.add(
      new TyphoonCircleEntity({
        typhoon,
        parent,
        axisType: "ShortestRadius30",
        material: Color.fromCssColorString("#0070FF90"),
        zIndex: 2,
      })
    );
    SOURCE.entities.add(
      new TyphoonCircleEntity({
        typhoon,
        parent,
        axisType: "LongestRadius50",
        material: Color.fromCssColorString("#E3716A50"),
        zIndex: 3,
      })
    );
    SOURCE.entities.add(
      new TyphoonCircleEntity({
        typhoon,
        parent,
        axisType: "ShortestRadius50",
        material: Color.fromCssColorString("#E3716A90"),
        zIndex: 4,
      })
    );
    return parent;
  });
};

export const init = async (widget: Viewer, typhoonDatas: Typhoon[]) => {
  WIDGET = widget;
  const dataSource = new CustomDataSource("typhoon");
  SOURCE = await widget.dataSources.add(dataSource);
  addTyphoon(typhoonDatas);
};

export const refresh = async (typhoonDatas: Typhoon[]) => {
  if (!SOURCE) return;
  clear();
  addTyphoon(typhoonDatas);
};

export const select = (date: Date) => {
  const { timeline, clock } = WIDGET;
  const newStart = JulianDate.fromDate(date);
  clock.startTime = newStart.clone();
  clock.currentTime = newStart.clone();
  clock.multiplier = 130000;
  timeline.zoomTo(newStart.clone(), clock.stopTime);
};

export const clear = () => {
  if (!(SOURCE && WIDGET)) return;
  SOURCE.entities?.removeAll();
  WIDGET.entities?.removeAll();
};
