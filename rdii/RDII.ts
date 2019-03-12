import "TimeSeries"

interface Rain {
    startDate: Date;
    endDate: Date;
}

interface IAndI {
    rainIntensities: number[];
    flowMax: number[]
}

class RDII {

    rainfall: TimeSeries;
    flow: TimeSeries;

    constructor(rainfall: TimeSeries, flow: TimeSeries) {
        this.rainfall = rainfall;
        this.flow = flow;
    }

    rains(): Rain[] {
        return [];
    }

    // Find Dry Weather Pattern for the given day
    dryWeatherPattern(day: Date): TimeSeries {
        return this.flow;
    }

    // Create RDII analysis for the given time range
    rdii(from: Date, to: Date): TimeSeries[] {
        return []
    }

    // Calculate I&I analysis for the whole data
    iandi(): IAndI {
        return {'rainIntensities': [], "flowMax": []};
    }
}