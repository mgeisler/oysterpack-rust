Feature: [01D3J441N6BM05NKCBQEVYTZY8] Gathering metrics

  Metrics can be gathered for all metric collectors.
  Metrics can be gathered for specified descriptors via the descriptor ID or name.
  Metrics can be gathered for specified MetricId(s).

  Background:
    Given [01D3J441N6BM05NKCBQEVYTZY8] metrics are registered for the following types:
      | MetricType     |
      | IntCounter     |
      | Counter        |
      | CounterVec     |
      | IntGauge       |
      | Gauge          |
      | GaugeVec       |
      | Histogram      |
      | HistogramTimer |
      | HistogramVec   |

  Scenario: [01D3PPPT1ZNXPKKWM29R14V5ZT] Gathering all metrics
    When [01D3PPPT1ZNXPKKWM29R14V5ZT-2] all metrics are gathered
    Then [01D3PPPT1ZNXPKKWM29R14V5ZT-3] metrics are returned for all registered metric descriptors

  Scenario: [01D3PPY3E710BYY8DQDKVQ31KY] Gather metrics for specified Desc IDs
    When [01D3PPY3E710BYY8DQDKVQ31KY-2] metrics are gathered for specified Desc IDs
    Then [01D3PPY3E710BYY8DQDKVQ31KY-3] metrics are returned for specified Desc IDs

  Scenario: [01D3PQ2KMBY07K48Q281SMPED6] Gather metrics for specified Desc.fq_name(s)
    When [01D3PQ2KMBY07K48Q281SMPED6-2] metrics are gathered
    Then [01D3PQ2KMBY07K48Q281SMPED6-3] metrics are returned for specified Desc.fq_name(s)

  Scenario: [01D3VC85Q8MVBJ543SHZ4RE9T2] Gather metrics for specified MetricId(s)
    When [01D3VC85Q8MVBJ543SHZ4RE9T2-2] metrics are gathered
    Then [01D3VC85Q8MVBJ543SHZ4RE9T2-3] metrics are returned for specified MetricId(s)