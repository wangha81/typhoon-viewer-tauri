import {
  buildModuleUrl,
  Cartesian3,
  TileMapServiceImageryProvider,
  Viewer,
} from "cesium";

export const createWidget = async (
  container: Element,
  creditContainer?: Element
) => {
  const imageryProvider = new TileMapServiceImageryProvider({
    url: buildModuleUrl("Assets/Textures/NaturalEarthII"),
  });
  const widget = new Viewer(container, {
    imageryProvider,
    creditContainer,
    homeButton: false,
    geocoder: false,
    infoBox: false,
    navigationHelpButton: false,
    sceneModePicker: false,
    scene3DOnly: true,
  });
  const [cwc] = Array.from(
    creditContainer?.getElementsByClassName("cesium-widget-credits")!
  );
  (cwc as HTMLDivElement).style.display = "none";
  widget.scene.globe.enableLighting = false;
  widget.scene.globe.showGroundAtmosphere = false;
  widget.camera.setView({
    destination : Cartesian3.fromDegrees(121, 21, 10000000)
});
  return widget;
};

