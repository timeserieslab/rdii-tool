//
// TimeSeries class assumes that the time series is evenly spaces and have no gaps (missing values)
//

class TimeSeries {

    startDate: Date;
    resolution: number;
    data: number[];

    constructor(startTime: Date, resolution: number, data: number[]) {
        this.startDate = startTime;
        this.resolution = resolution;
        this.data = data;
    }
}