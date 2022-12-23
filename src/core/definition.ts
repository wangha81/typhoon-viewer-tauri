export type Header = {
    Indicator: string
    InternationalNumberID: string
    Lines: number
    TropicalCycloneNumberID?: string
    InternationalNumberIDRp: string
    Flag: string
    TimeDelta: number
    Name?: string
    Date: string
    Last: string
    last?: Date
}

export type Point = {
    Time: string
    Indicator: string
    Grade: string
    Latitude: number
    Longitude: number
    Pressure: number
    MaxWindSpeed?: number
    Direction50?: number
    LongestRadius50?: number
    ShortestRadius50?: number
    Direction30?: number
    LongestRadius30?: number
    ShortestRadius30?: number
}

export type Typhoon = {
    header: Header
    points: Point[]
}
