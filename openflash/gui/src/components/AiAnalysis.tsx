import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./AiAnalysis.css";

interface PatternInfo {
  pattern_type: string;
  start_offset: number;
  end_offset: number;
  confidence: string;
  description: string;
}

interface AnomalyInfo {
  severity: string;
  location: number | null;
  description: string;
  recommendation: string;
}

interface RecoverySuggestionInfo {
  priority: number;
  action: string;
  description: string;
  estimated_success: number;
}

interface ChipRecommendationInfo {
  category: string;
  title: string;
  description: string;
  importance: number;
}

interface AiAnalysisResponse {
  patterns: PatternInfo[];
  anomalies: AnomalyInfo[];
  recovery_suggestions: RecoverySuggestionInfo[];
  chip_recommendations: ChipRecommendationInfo[];
  data_quality_score: number;
  encryption_probability: number;
  compression_probability: number;
  summary: string;
}

interface AiAnalysisProps {
  data: Uint8Array | null;
  pageSize: number;
  pagesPerBlock: number;
  onPatternSelect?: (offset: number) => void;
}

export function AiAnalysis({ data, pageSize, pagesPerBlock, onPatternSelect }: AiAnalysisProps) {
  const [analysis, setAnalysis] = useState<AiAnalysisResponse | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activeSection, setActiveSection] = useState<string>("overview");

  useEffect(() => {
    if (data && data.length > 0) {
      runAnalysis();
    }
  }, [data]);

  async function runAnalysis() {
    if (!data) return;
    
    setIsAnalyzing(true);
    setError(null);
    
    try {
      const result = await invoke<AiAnalysisResponse>("ai_analyze_dump", {
        data: Array.from(data),
        pageSize,
        pagesPerBlock,
      });
      setAnalysis(result);
    } catch (e) {
      setError(`Analysis failed: ${e}`);
    } finally {
      setIsAnalyzing(false);
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  }

  function getPatternIcon(type: string): string {
    const icons: Record<string, string> = {
      "Encrypted": "🔐",
      "Compressed": "📦",
      "Executable": "⚙️",
      "Text": "📝",
      "Empty": "⬜",
      "Zeroed": "0️⃣",
      "Repeating": "🔄",
      "StructuredBinary": "📊",
      "FilesystemMeta": "📁",
      "Random": "🎲",
    };
    return icons[type] || "❓";
  }

  function getSeverityColor(severity: string): string {
    switch (severity) {
      case "Critical": return "#ff4444";
      case "Warning": return "#ffaa00";
      case "Info": return "#4488ff";
      default: return "#888";
    }
  }

  function getConfidenceColor(confidence: string): string {
    switch (confidence) {
      case "VeryHigh": return "#00cc66";
      case "High": return "#88cc00";
      case "Medium": return "#ffaa00";
      case "Low": return "#ff6644";
      default: return "#888";
    }
  }

  if (!data) {
    return (
      <div className="ai-analysis empty">
        <div className="ai-empty-state">
          <span className="ai-icon">🤖</span>
          <h3>AI Analysis</h3>
          <p>Load or dump data to enable AI-powered analysis</p>
        </div>
      </div>
    );
  }

  if (isAnalyzing) {
    return (
      <div className="ai-analysis loading">
        <div className="ai-loading-state">
          <div className="ai-spinner"></div>
          <h3>Analyzing...</h3>
          <p>AI is examining {formatBytes(data.length)} of data</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="ai-analysis error">
        <div className="ai-error-state">
          <span className="ai-icon">⚠️</span>
          <h3>Analysis Error</h3>
          <p>{error}</p>
          <button onClick={runAnalysis}>Retry</button>
        </div>
      </div>
    );
  }

  if (!analysis) {
    return null;
  }

  return (
    <div className="ai-analysis">
      {/* Summary Card */}
      <div className="ai-summary-card">
        <div className="ai-summary-header">
          <span className="ai-icon">🤖</span>
          <h3>AI Analysis Summary</h3>
          <button className="ai-refresh" onClick={runAnalysis} title="Re-analyze">
            🔄
          </button>
        </div>
        <p className="ai-summary-text">{analysis.summary}</p>
        
        <div className="ai-metrics">
          <div className="ai-metric">
            <label>Data Quality</label>
            <div className="ai-progress-bar">
              <div 
                className="ai-progress-fill quality"
                style={{ width: `${analysis.data_quality_score * 100}%` }}
              />
            </div>
            <span>{Math.round(analysis.data_quality_score * 100)}%</span>
          </div>
          
          <div className="ai-metric">
            <label>Encryption</label>
            <div className="ai-progress-bar">
              <div 
                className="ai-progress-fill encryption"
                style={{ width: `${analysis.encryption_probability * 100}%` }}
              />
            </div>
            <span>{Math.round(analysis.encryption_probability * 100)}%</span>
          </div>
          
          <div className="ai-metric">
            <label>Compression</label>
            <div className="ai-progress-bar">
              <div 
                className="ai-progress-fill compression"
                style={{ width: `${analysis.compression_probability * 100}%` }}
              />
            </div>
            <span>{Math.round(analysis.compression_probability * 100)}%</span>
          </div>
        </div>
      </div>

      {/* Section Tabs */}
      <div className="ai-tabs">
        <button 
          className={activeSection === "overview" ? "active" : ""}
          onClick={() => setActiveSection("overview")}
        >
          📊 Patterns ({analysis.patterns.length})
        </button>
        <button 
          className={activeSection === "anomalies" ? "active" : ""}
          onClick={() => setActiveSection("anomalies")}
        >
          ⚠️ Issues ({analysis.anomalies.length})
        </button>
        <button 
          className={activeSection === "recovery" ? "active" : ""}
          onClick={() => setActiveSection("recovery")}
        >
          🔧 Recovery ({analysis.recovery_suggestions.length})
        </button>
        <button 
          className={activeSection === "recommendations" ? "active" : ""}
          onClick={() => setActiveSection("recommendations")}
        >
          💡 Tips ({analysis.chip_recommendations.length})
        </button>
      </div>

      {/* Section Content */}
      <div className="ai-section-content">
        {activeSection === "overview" && (
          <div className="ai-patterns">
            {analysis.patterns.length === 0 ? (
              <p className="ai-empty">No patterns detected</p>
            ) : (
              <div className="ai-pattern-list">
                {analysis.patterns.map((pattern, i) => (
                  <div 
                    key={i} 
                    className="ai-pattern-item"
                    onClick={() => onPatternSelect?.(pattern.start_offset)}
                  >
                    <span className="pattern-icon">
                      {getPatternIcon(pattern.pattern_type)}
                    </span>
                    <div className="pattern-info">
                      <div className="pattern-header">
                        <span className="pattern-type">{pattern.pattern_type}</span>
                        <span 
                          className="pattern-confidence"
                          style={{ color: getConfidenceColor(pattern.confidence) }}
                        >
                          {pattern.confidence}
                        </span>
                      </div>
                      <p className="pattern-desc">{pattern.description}</p>
                      <div className="pattern-range">
                        <span>0x{pattern.start_offset.toString(16).toUpperCase()}</span>
                        <span>→</span>
                        <span>0x{pattern.end_offset.toString(16).toUpperCase()}</span>
                        <span className="pattern-size">
                          ({formatBytes(pattern.end_offset - pattern.start_offset)})
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeSection === "anomalies" && (
          <div className="ai-anomalies">
            {analysis.anomalies.length === 0 ? (
              <p className="ai-empty">✅ No issues detected</p>
            ) : (
              <div className="ai-anomaly-list">
                {analysis.anomalies.map((anomaly, i) => (
                  <div 
                    key={i} 
                    className="ai-anomaly-item"
                    style={{ borderLeftColor: getSeverityColor(anomaly.severity) }}
                  >
                    <div className="anomaly-header">
                      <span 
                        className="anomaly-severity"
                        style={{ color: getSeverityColor(anomaly.severity) }}
                      >
                        {anomaly.severity}
                      </span>
                      {anomaly.location !== null && (
                        <span className="anomaly-location">
                          @ 0x{anomaly.location.toString(16).toUpperCase()}
                        </span>
                      )}
                    </div>
                    <p className="anomaly-desc">{anomaly.description}</p>
                    <p className="anomaly-recommendation">
                      💡 {anomaly.recommendation}
                    </p>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeSection === "recovery" && (
          <div className="ai-recovery">
            {analysis.recovery_suggestions.length === 0 ? (
              <p className="ai-empty">No recovery actions needed</p>
            ) : (
              <div className="ai-recovery-list">
                {analysis.recovery_suggestions.map((suggestion, i) => (
                  <div key={i} className="ai-recovery-item">
                    <div className="recovery-header">
                      <span className="recovery-priority">#{suggestion.priority}</span>
                      <span className="recovery-action">{suggestion.action}</span>
                      <span className="recovery-success">
                        {Math.round(suggestion.estimated_success * 100)}% success
                      </span>
                    </div>
                    <p className="recovery-desc">{suggestion.description}</p>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {activeSection === "recommendations" && (
          <div className="ai-recommendations">
            {analysis.chip_recommendations.length === 0 ? (
              <p className="ai-empty">No recommendations</p>
            ) : (
              <div className="ai-recommendation-list">
                {analysis.chip_recommendations.map((rec, i) => (
                  <div key={i} className="ai-recommendation-item">
                    <div className="recommendation-header">
                      <span className="recommendation-category">{rec.category}</span>
                      <span className="recommendation-importance">
                        {"⭐".repeat(Math.min(rec.importance / 2, 5))}
                      </span>
                    </div>
                    <h4 className="recommendation-title">{rec.title}</h4>
                    <p className="recommendation-desc">{rec.description}</p>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
