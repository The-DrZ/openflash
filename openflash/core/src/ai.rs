//! AI-powered analysis module for OpenFlash v1.3
//! 
//! Provides intelligent analysis, pattern recognition, and recommendations
//! for NAND/eMMC flash memory dumps.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Data Structures
// ============================================================================

/// AI analysis confidence level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Confidence {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Confidence {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.9 => Confidence::VeryHigh,
            s if s >= 0.7 => Confidence::High,
            s if s >= 0.5 => Confidence::Medium,
            _ => Confidence::Low,
        }
    }
    
    pub fn to_score(&self) -> f32 {
        match self {
            Confidence::VeryHigh => 0.95,
            Confidence::High => 0.8,
            Confidence::Medium => 0.6,
            Confidence::Low => 0.3,
        }
    }
}

/// Detected data pattern type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternType {
    /// Encrypted data (high entropy, no structure)
    Encrypted,
    /// Compressed data (high entropy with headers)
    Compressed,
    /// Executable code (specific byte patterns)
    Executable,
    /// Text/ASCII data
    Text,
    /// Structured binary (headers, tables)
    StructuredBinary,
    /// Empty/erased (0xFF)
    Empty,
    /// Zero-filled
    Zeroed,
    /// Repeating pattern
    Repeating,
    /// Random/corrupted
    Random,
    /// Filesystem metadata
    FilesystemMeta,
}

/// Detected pattern in dump
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub pattern_type: PatternType,
    pub start_offset: usize,
    pub end_offset: usize,
    pub confidence: Confidence,
    pub description: String,
    pub details: HashMap<String, String>,
}

/// Anomaly severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Info,
    Warning,
    Critical,
}

/// Detected anomaly in dump
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub severity: AnomalySeverity,
    pub location: Option<usize>,
    pub description: String,
    pub recommendation: String,
}

/// Recovery suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySuggestion {
    pub priority: u8,
    pub action: String,
    pub description: String,
    pub estimated_success: f32,
    pub affected_regions: Vec<(usize, usize)>,
}

/// Chip-specific recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChipRecommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub importance: u8,
}

/// Complete AI analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResult {
    pub patterns: Vec<DetectedPattern>,
    pub anomalies: Vec<Anomaly>,
    pub recovery_suggestions: Vec<RecoverySuggestion>,
    pub chip_recommendations: Vec<ChipRecommendation>,
    pub data_quality_score: f32,
    pub encryption_probability: f32,
    pub compression_probability: f32,
    pub summary: String,
}


// ============================================================================
// AI Analyzer
// ============================================================================

/// AI-powered analyzer for flash memory dumps
pub struct AiAnalyzer {
    page_size: usize,
    block_size: usize,
}

impl Default for AiAnalyzer {
    fn default() -> Self {
        Self::new(2048, 64)
    }
}

impl AiAnalyzer {
    pub fn new(page_size: usize, pages_per_block: usize) -> Self {
        Self {
            page_size,
            block_size: pages_per_block,
        }
    }

    /// Perform complete AI analysis on dump data
    pub fn analyze(&self, data: &[u8]) -> AiAnalysisResult {
        let patterns = self.detect_patterns(data);
        let anomalies = self.detect_anomalies(data, &patterns);
        let recovery_suggestions = self.generate_recovery_suggestions(data, &anomalies);
        let chip_recommendations = self.generate_chip_recommendations(data, &patterns);
        
        let data_quality_score = self.calculate_data_quality(data, &anomalies);
        let encryption_probability = self.estimate_encryption_probability(data, &patterns);
        let compression_probability = self.estimate_compression_probability(data, &patterns);
        
        let summary = self.generate_summary(
            &patterns, 
            &anomalies, 
            data_quality_score,
            encryption_probability,
        );

        AiAnalysisResult {
            patterns,
            anomalies,
            recovery_suggestions,
            chip_recommendations,
            data_quality_score,
            encryption_probability,
            compression_probability,
            summary,
        }
    }

    // ========================================================================
    // Pattern Detection
    // ========================================================================

    /// Detect patterns in dump data
    pub fn detect_patterns(&self, data: &[u8]) -> Vec<DetectedPattern> {
        let mut patterns: Vec<DetectedPattern> = Vec::new();
        let mut offset = 0;
        
        while offset < data.len() {
            let chunk_size = (self.page_size * 4).min(data.len() - offset);
            let chunk = &data[offset..offset + chunk_size];
            
            if let Some(pattern) = self.analyze_chunk(chunk, offset) {
                // Merge with previous pattern if same type
                if let Some(last) = patterns.last_mut() {
                    if last.pattern_type == pattern.pattern_type 
                        && last.end_offset == offset 
                    {
                        last.end_offset = pattern.end_offset;
                        offset += chunk_size;
                        continue;
                    }
                }
                patterns.push(pattern);
            }
            
            offset += chunk_size;
        }
        
        patterns
    }

    fn analyze_chunk(&self, chunk: &[u8], offset: usize) -> Option<DetectedPattern> {
        let entropy = self.calculate_entropy(chunk);
        let zero_ratio = chunk.iter().filter(|&&b| b == 0x00).count() as f32 / chunk.len() as f32;
        let ff_ratio = chunk.iter().filter(|&&b| b == 0xFF).count() as f32 / chunk.len() as f32;
        
        // Check for empty/erased pages
        if ff_ratio > 0.99 {
            return Some(DetectedPattern {
                pattern_type: PatternType::Empty,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::VeryHigh,
                description: "Erased/empty region (0xFF)".to_string(),
                details: HashMap::new(),
            });
        }
        
        // Check for zero-filled
        if zero_ratio > 0.99 {
            return Some(DetectedPattern {
                pattern_type: PatternType::Zeroed,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::VeryHigh,
                description: "Zero-filled region".to_string(),
                details: HashMap::new(),
            });
        }
        
        // Check for repeating patterns
        if let Some(pattern) = self.detect_repeating_pattern(chunk, offset) {
            return Some(pattern);
        }
        
        // Check for text data
        let printable_ratio = chunk.iter()
            .filter(|&&b| (0x20..=0x7E).contains(&b) || b == 0x0A || b == 0x0D || b == 0x09)
            .count() as f32 / chunk.len() as f32;
        
        if printable_ratio > 0.85 {
            return Some(DetectedPattern {
                pattern_type: PatternType::Text,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::from_score(printable_ratio),
                description: "ASCII/text data region".to_string(),
                details: HashMap::new(),
            });
        }
        
        // Check for compression signatures
        if let Some(pattern) = self.detect_compression(chunk, offset) {
            return Some(pattern);
        }
        
        // Check for executable code patterns
        if let Some(pattern) = self.detect_executable(chunk, offset) {
            return Some(pattern);
        }
        
        // High entropy without structure = likely encrypted
        if entropy > 7.5 {
            let mut details = HashMap::new();
            details.insert("entropy".to_string(), format!("{:.2}", entropy));
            
            return Some(DetectedPattern {
                pattern_type: PatternType::Encrypted,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::from_score(((entropy - 7.0) / 1.0) as f32),
                description: "High-entropy data (likely encrypted)".to_string(),
                details,
            });
        }
        
        // Medium-high entropy = compressed or structured
        if entropy > 5.0 {
            return Some(DetectedPattern {
                pattern_type: PatternType::StructuredBinary,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::Medium,
                description: "Structured binary data".to_string(),
                details: HashMap::new(),
            });
        }
        
        None
    }

    fn detect_repeating_pattern(&self, chunk: &[u8], offset: usize) -> Option<DetectedPattern> {
        // Check for short repeating patterns (2-16 bytes)
        for pattern_len in 2..=16 {
            if chunk.len() < pattern_len * 4 {
                continue;
            }
            
            let pattern = &chunk[..pattern_len];
            let mut matches = 0;
            let mut total = 0;
            
            for i in (0..chunk.len()).step_by(pattern_len) {
                if i + pattern_len <= chunk.len() {
                    total += 1;
                    if &chunk[i..i + pattern_len] == pattern {
                        matches += 1;
                    }
                }
            }
            
            let match_ratio = matches as f32 / total as f32;
            if match_ratio > 0.9 {
                let mut details = HashMap::new();
                details.insert("pattern_length".to_string(), pattern_len.to_string());
                details.insert("pattern_hex".to_string(), 
                    pattern.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "));
                
                return Some(DetectedPattern {
                    pattern_type: PatternType::Repeating,
                    start_offset: offset,
                    end_offset: offset + chunk.len(),
                    confidence: Confidence::from_score(match_ratio),
                    description: format!("Repeating {}-byte pattern", pattern_len),
                    details,
                });
            }
        }
        
        None
    }

    fn detect_compression(&self, chunk: &[u8], offset: usize) -> Option<DetectedPattern> {
        let signatures = [
            (&[0x1F, 0x8B][..], "gzip"),
            (&[0x78, 0x9C][..], "zlib"),
            (&[0x78, 0xDA][..], "zlib (best)"),
            (&[0x5D, 0x00, 0x00][..], "LZMA"),
            (&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00][..], "XZ"),
            (&[0x28, 0xB5, 0x2F, 0xFD][..], "Zstandard"),
            (&[0x04, 0x22, 0x4D, 0x18][..], "LZ4"),
        ];
        
        for (sig, name) in signatures {
            if chunk.len() >= sig.len() && &chunk[..sig.len()] == sig {
                let mut details = HashMap::new();
                details.insert("format".to_string(), name.to_string());
                
                return Some(DetectedPattern {
                    pattern_type: PatternType::Compressed,
                    start_offset: offset,
                    end_offset: offset + chunk.len(),
                    confidence: Confidence::High,
                    description: format!("{} compressed data", name),
                    details,
                });
            }
        }
        
        None
    }

    fn detect_executable(&self, chunk: &[u8], offset: usize) -> Option<DetectedPattern> {
        // ARM thumb instructions often start with specific patterns
        // Check for common ARM/MIPS patterns
        
        if chunk.len() < 4 {
            return None;
        }
        
        // ELF header
        if chunk.len() >= 4 && &chunk[..4] == b"\x7FELF" {
            return Some(DetectedPattern {
                pattern_type: PatternType::Executable,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::VeryHigh,
                description: "ELF executable".to_string(),
                details: HashMap::new(),
            });
        }
        
        // U-Boot image
        if chunk.len() >= 4 && chunk[..4] == [0x27, 0x05, 0x19, 0x56] {
            return Some(DetectedPattern {
                pattern_type: PatternType::Executable,
                start_offset: offset,
                end_offset: offset + chunk.len(),
                confidence: Confidence::VeryHigh,
                description: "U-Boot image header".to_string(),
                details: HashMap::new(),
            });
        }
        
        None
    }


    // ========================================================================
    // Anomaly Detection
    // ========================================================================

    /// Detect anomalies in dump data
    pub fn detect_anomalies(&self, data: &[u8], patterns: &[DetectedPattern]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        // Check for bad block markers
        anomalies.extend(self.detect_bad_block_anomalies(data));
        
        // Check for ECC errors (bit flips)
        anomalies.extend(self.detect_bit_flip_anomalies(data));
        
        // Check for truncated data
        if let Some(anomaly) = self.detect_truncation(data, patterns) {
            anomalies.push(anomaly);
        }
        
        // Check for corrupted headers
        anomalies.extend(self.detect_header_corruption(data, patterns));
        
        // Check for unusual pattern transitions
        anomalies.extend(self.detect_pattern_anomalies(patterns));
        
        // Sort by severity
        anomalies.sort_by(|a, b| {
            let severity_order = |s: &AnomalySeverity| match s {
                AnomalySeverity::Critical => 0,
                AnomalySeverity::Warning => 1,
                AnomalySeverity::Info => 2,
            };
            severity_order(&a.severity).cmp(&severity_order(&b.severity))
        });
        
        anomalies
    }

    fn detect_bad_block_anomalies(&self, data: &[u8]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        let block_bytes = self.page_size * self.block_size;
        let mut bad_blocks = Vec::new();
        
        for (block_num, chunk) in data.chunks(block_bytes).enumerate() {
            // Check first byte of first page (common bad block marker location)
            if !chunk.is_empty() && chunk[0] != 0xFF {
                // Check if it looks like a bad block marker
                if chunk.len() > self.page_size {
                    let spare_start = self.page_size;
                    if spare_start < chunk.len() && chunk[spare_start] != 0xFF {
                        bad_blocks.push(block_num);
                    }
                }
            }
        }
        
        if !bad_blocks.is_empty() {
            let severity = if bad_blocks.len() > 10 {
                AnomalySeverity::Warning
            } else {
                AnomalySeverity::Info
            };
            
            anomalies.push(Anomaly {
                severity,
                location: None,
                description: format!("Found {} potential bad blocks", bad_blocks.len()),
                recommendation: "Bad blocks are normal for NAND flash. Consider using ECC and bad block management.".to_string(),
            });
        }
        
        anomalies
    }

    fn detect_bit_flip_anomalies(&self, data: &[u8]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        let mut suspicious_pages = 0;
        
        for (page_num, page) in data.chunks(self.page_size).enumerate() {
            // Count bytes that are almost 0xFF (single bit flip)
            let almost_ff = page.iter()
                .filter(|&&b| b != 0xFF && (b | (b + 1)) == 0xFF)
                .count();
            
            // Count bytes that are almost 0x00 (single bit flip)
            let almost_00 = page.iter()
                .filter(|&&b| b != 0x00 && b.count_ones() == 1)
                .count();
            
            if almost_ff > 10 || almost_00 > 10 {
                suspicious_pages += 1;
            }
        }
        
        if suspicious_pages > 0 {
            let severity = if suspicious_pages > data.len() / self.page_size / 10 {
                AnomalySeverity::Warning
            } else {
                AnomalySeverity::Info
            };
            
            anomalies.push(Anomaly {
                severity,
                location: None,
                description: format!("{} pages show signs of bit rot/ECC errors", suspicious_pages),
                recommendation: "Apply ECC correction to recover data. Consider re-reading with different timing.".to_string(),
            });
        }
        
        anomalies
    }

    fn detect_truncation(&self, data: &[u8], patterns: &[DetectedPattern]) -> Option<Anomaly> {
        // Check if dump ends abruptly in the middle of data
        if data.len() < self.page_size {
            return Some(Anomaly {
                severity: AnomalySeverity::Critical,
                location: Some(data.len()),
                description: "Dump appears truncated (less than one page)".to_string(),
                recommendation: "Re-dump the chip ensuring complete read operation.".to_string(),
            });
        }
        
        // Check if last pattern is incomplete
        if let Some(last) = patterns.last() {
            if last.pattern_type != PatternType::Empty 
                && last.end_offset == data.len() 
                && data.len() % (self.page_size * self.block_size) != 0 
            {
                return Some(Anomaly {
                    severity: AnomalySeverity::Warning,
                    location: Some(data.len()),
                    description: "Dump may be truncated (doesn't end on block boundary)".to_string(),
                    recommendation: "Verify dump size matches expected chip capacity.".to_string(),
                });
            }
        }
        
        None
    }

    fn detect_header_corruption(&self, data: &[u8], patterns: &[DetectedPattern]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        for pattern in patterns {
            if pattern.pattern_type == PatternType::Compressed {
                // Verify compression header integrity
                let header_data = &data[pattern.start_offset..pattern.start_offset.min(data.len())];
                
                // Check for common corruption patterns
                if header_data.len() >= 10 {
                    let entropy = self.calculate_entropy(&header_data[..10]);
                    if entropy < 2.0 {
                        anomalies.push(Anomaly {
                            severity: AnomalySeverity::Warning,
                            location: Some(pattern.start_offset),
                            description: format!("Compressed data header at 0x{:X} may be corrupted", pattern.start_offset),
                            recommendation: "Try alternative decompression tools or manual header repair.".to_string(),
                        });
                    }
                }
            }
        }
        
        anomalies
    }

    fn detect_pattern_anomalies(&self, patterns: &[DetectedPattern]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        
        // Check for suspicious pattern transitions
        for window in patterns.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            
            // Encrypted data followed immediately by text is suspicious
            if prev.pattern_type == PatternType::Encrypted 
                && curr.pattern_type == PatternType::Text 
                && curr.start_offset - prev.end_offset < 16 
            {
                anomalies.push(Anomaly {
                    severity: AnomalySeverity::Info,
                    location: Some(prev.end_offset),
                    description: format!("Unusual transition from encrypted to text at 0x{:X}", prev.end_offset),
                    recommendation: "May indicate encryption boundary or misidentified pattern.".to_string(),
                });
            }
        }
        
        // Check for fragmented empty regions (sign of wear)
        let empty_count = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Empty)
            .count();
        
        if empty_count > 10 && patterns.len() > 20 {
            anomalies.push(Anomaly {
                severity: AnomalySeverity::Info,
                location: None,
                description: format!("Highly fragmented empty space ({} regions)", empty_count),
                recommendation: "May indicate heavy wear or deleted data. Consider forensic analysis.".to_string(),
            });
        }
        
        anomalies
    }


    // ========================================================================
    // Recovery Suggestions
    // ========================================================================

    /// Generate data recovery suggestions
    pub fn generate_recovery_suggestions(
        &self, 
        data: &[u8], 
        anomalies: &[Anomaly]
    ) -> Vec<RecoverySuggestion> {
        let mut suggestions = Vec::new();
        
        // Check for ECC-correctable errors
        let bit_flip_anomaly = anomalies.iter()
            .any(|a| a.description.contains("bit rot") || a.description.contains("ECC"));
        
        if bit_flip_anomaly {
            suggestions.push(RecoverySuggestion {
                priority: 1,
                action: "Apply ECC Correction".to_string(),
                description: "Use BCH or Hamming ECC to correct bit errors in affected pages.".to_string(),
                estimated_success: 0.85,
                affected_regions: vec![(0, data.len())],
            });
        }
        
        // Check for bad blocks
        let bad_block_anomaly = anomalies.iter()
            .any(|a| a.description.contains("bad block"));
        
        if bad_block_anomaly {
            suggestions.push(RecoverySuggestion {
                priority: 2,
                action: "Skip Bad Blocks".to_string(),
                description: "Reconstruct data by skipping marked bad blocks and adjusting offsets.".to_string(),
                estimated_success: 0.90,
                affected_regions: vec![],
            });
        }
        
        // Check for truncation
        let truncation_anomaly = anomalies.iter()
            .any(|a| a.description.contains("truncated"));
        
        if truncation_anomaly {
            suggestions.push(RecoverySuggestion {
                priority: 1,
                action: "Re-dump Chip".to_string(),
                description: "Perform a fresh dump ensuring stable connection and complete read.".to_string(),
                estimated_success: 0.95,
                affected_regions: vec![(data.len().saturating_sub(self.page_size), data.len())],
            });
        }
        
        // General suggestions based on data analysis
        let entropy = self.calculate_entropy(data);
        
        if entropy > 7.0 {
            suggestions.push(RecoverySuggestion {
                priority: 3,
                action: "Identify Encryption".to_string(),
                description: "High entropy suggests encryption. Try to identify encryption scheme and locate keys.".to_string(),
                estimated_success: 0.30,
                affected_regions: vec![(0, data.len())],
            });
        }
        
        // Sort by priority
        suggestions.sort_by_key(|s| s.priority);
        
        suggestions
    }

    // ========================================================================
    // Chip Recommendations
    // ========================================================================

    /// Generate chip-specific recommendations
    pub fn generate_chip_recommendations(
        &self, 
        data: &[u8],
        patterns: &[DetectedPattern]
    ) -> Vec<ChipRecommendation> {
        let mut recommendations = Vec::new();
        
        // Page size recommendation
        let detected_page_size = self.detect_likely_page_size(data);
        if detected_page_size != self.page_size {
            recommendations.push(ChipRecommendation {
                category: "Configuration".to_string(),
                title: "Page Size Mismatch".to_string(),
                description: format!(
                    "Data patterns suggest page size of {} bytes, but {} is configured. Consider adjusting.",
                    detected_page_size, self.page_size
                ),
                importance: 8,
            });
        }
        
        // ECC recommendation based on data quality
        let has_errors = patterns.iter().any(|p| {
            matches!(p.pattern_type, PatternType::Random)
        });
        
        if has_errors {
            recommendations.push(ChipRecommendation {
                category: "ECC".to_string(),
                title: "Enable ECC Correction".to_string(),
                description: "Data shows signs of bit errors. Enable BCH-8 or BCH-16 ECC for better recovery.".to_string(),
                importance: 9,
            });
        }
        
        // Read timing recommendation
        let empty_ratio = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Empty)
            .map(|p| p.end_offset - p.start_offset)
            .sum::<usize>() as f32 / data.len() as f32;
        
        if empty_ratio > 0.8 {
            recommendations.push(ChipRecommendation {
                category: "Timing".to_string(),
                title: "Verify Read Timing".to_string(),
                description: "Large empty regions detected. Verify chip timing parameters and try slower read speed.".to_string(),
                importance: 7,
            });
        }
        
        // Filesystem extraction recommendation
        let has_filesystem = patterns.iter().any(|p| {
            matches!(p.pattern_type, PatternType::Compressed | PatternType::Executable)
        });
        
        if has_filesystem {
            recommendations.push(ChipRecommendation {
                category: "Analysis".to_string(),
                title: "Extract Filesystem".to_string(),
                description: "Compressed/executable data detected. Use binwalk or similar tool to extract filesystem contents.".to_string(),
                importance: 6,
            });
        }
        
        // Sort by importance
        recommendations.sort_by(|a, b| b.importance.cmp(&a.importance));
        
        recommendations
    }

    fn detect_likely_page_size(&self, data: &[u8]) -> usize {
        // Analyze data alignment patterns to detect page size
        let candidates = [512, 2048, 4096, 8192, 16384];
        let mut best_size = self.page_size;
        let mut best_score = 0.0f32;
        
        for &size in &candidates {
            if data.len() < size * 4 {
                continue;
            }
            
            let mut alignment_score = 0.0;
            let mut samples = 0;
            
            for offset in (0..data.len()).step_by(size) {
                if offset + 16 > data.len() {
                    break;
                }
                
                // Check for page-aligned patterns (headers, 0xFF boundaries)
                let chunk = &data[offset..offset + 16];
                
                // Signature at page boundary
                if chunk[..4] == [0x27, 0x05, 0x19, 0x56]  // U-Boot
                    || chunk[..4] == *b"hsqs"  // SquashFS
                    || chunk[..4] == *b"\x7FELF"  // ELF
                    || chunk.iter().all(|&b| b == 0xFF)  // Empty page start
                {
                    alignment_score += 1.0;
                }
                
                samples += 1;
            }
            
            if samples > 0 {
                let score = alignment_score / samples as f32;
                if score > best_score {
                    best_score = score;
                    best_size = size;
                }
            }
        }
        
        best_size
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    fn calculate_data_quality(&self, data: &[u8], anomalies: &[Anomaly]) -> f32 {
        let mut score = 1.0f32;
        
        // Deduct for anomalies
        for anomaly in anomalies {
            match anomaly.severity {
                AnomalySeverity::Critical => score -= 0.3,
                AnomalySeverity::Warning => score -= 0.1,
                AnomalySeverity::Info => score -= 0.02,
            }
        }
        
        // Check for excessive empty space
        let empty_ratio = data.iter().filter(|&&b| b == 0xFF).count() as f32 / data.len() as f32;
        if empty_ratio > 0.9 {
            score -= 0.2;
        }
        
        score.max(0.0).min(1.0)
    }

    fn estimate_encryption_probability(&self, data: &[u8], patterns: &[DetectedPattern]) -> f32 {
        let encrypted_bytes: usize = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Encrypted)
            .map(|p| p.end_offset - p.start_offset)
            .sum();
        
        let total_data_bytes: usize = patterns.iter()
            .filter(|p| !matches!(p.pattern_type, PatternType::Empty | PatternType::Zeroed))
            .map(|p| p.end_offset - p.start_offset)
            .sum();
        
        if total_data_bytes == 0 {
            return 0.0;
        }
        
        (encrypted_bytes as f32 / total_data_bytes as f32).min(1.0)
    }

    fn estimate_compression_probability(&self, data: &[u8], patterns: &[DetectedPattern]) -> f32 {
        let compressed_bytes: usize = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Compressed)
            .map(|p| p.end_offset - p.start_offset)
            .sum();
        
        let total_data_bytes: usize = patterns.iter()
            .filter(|p| !matches!(p.pattern_type, PatternType::Empty | PatternType::Zeroed))
            .map(|p| p.end_offset - p.start_offset)
            .sum();
        
        if total_data_bytes == 0 {
            return 0.0;
        }
        
        (compressed_bytes as f32 / total_data_bytes as f32).min(1.0)
    }

    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0u64; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    fn generate_summary(
        &self,
        patterns: &[DetectedPattern],
        anomalies: &[Anomaly],
        quality_score: f32,
        encryption_prob: f32,
    ) -> String {
        let mut parts = Vec::new();
        
        // Quality assessment
        let quality_desc = match quality_score {
            s if s >= 0.9 => "excellent",
            s if s >= 0.7 => "good",
            s if s >= 0.5 => "fair",
            _ => "poor",
        };
        parts.push(format!("Data quality: {} ({:.0}%)", quality_desc, quality_score * 100.0));
        
        // Pattern summary
        let pattern_counts: HashMap<&str, usize> = patterns.iter()
            .map(|p| match p.pattern_type {
                PatternType::Encrypted => "encrypted",
                PatternType::Compressed => "compressed",
                PatternType::Executable => "executable",
                PatternType::Text => "text",
                PatternType::Empty => "empty",
                _ => "other",
            })
            .fold(HashMap::new(), |mut acc, t| {
                *acc.entry(t).or_insert(0) += 1;
                acc
            });
        
        if !pattern_counts.is_empty() {
            let pattern_str: Vec<String> = pattern_counts.iter()
                .filter(|(k, _)| **k != "empty" && **k != "other")
                .map(|(k, v)| format!("{} {}", v, k))
                .collect();
            
            if !pattern_str.is_empty() {
                parts.push(format!("Found: {}", pattern_str.join(", ")));
            }
        }
        
        // Encryption warning
        if encryption_prob > 0.5 {
            parts.push(format!("⚠️ {:.0}% likely encrypted", encryption_prob * 100.0));
        }
        
        // Anomaly summary
        let critical = anomalies.iter().filter(|a| a.severity == AnomalySeverity::Critical).count();
        let warnings = anomalies.iter().filter(|a| a.severity == AnomalySeverity::Warning).count();
        
        if critical > 0 {
            parts.push(format!("🔴 {} critical issues", critical));
        }
        if warnings > 0 {
            parts.push(format!("🟡 {} warnings", warnings));
        }
        
        parts.join(". ")
    }
}


// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = AiAnalyzer::new(2048, 64);
        assert_eq!(analyzer.page_size, 2048);
    }

    #[test]
    fn test_empty_detection() {
        let analyzer = AiAnalyzer::default();
        let data = vec![0xFFu8; 8192];
        
        let patterns = analyzer.detect_patterns(&data);
        assert!(!patterns.is_empty());
        assert_eq!(patterns[0].pattern_type, PatternType::Empty);
    }

    #[test]
    fn test_zero_detection() {
        let analyzer = AiAnalyzer::default();
        let data = vec![0x00u8; 8192];
        
        let patterns = analyzer.detect_patterns(&data);
        assert!(!patterns.is_empty());
        assert_eq!(patterns[0].pattern_type, PatternType::Zeroed);
    }

    #[test]
    fn test_text_detection() {
        let analyzer = AiAnalyzer::new(512, 32);
        // Need enough text data to fill a chunk (page_size * 4 = 2048 bytes)
        let text = "Hello, this is a test string with lots of ASCII text content that should be detected as text data by the analyzer. The quick brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. This is additional text to ensure we have enough data for the pattern detection algorithm to work correctly. We need at least 2048 bytes of text data. Adding more text here to reach the required length. The pattern detection works on chunks of data, so we need sufficient text content. More text follows to pad the buffer. Testing one two three four five six seven eight nine ten. ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789. Special characters and punctuation: !@#$%^&*()_+-=[]{}|;':\",./<>? End of text padding. More padding text here. Even more text to ensure we have enough bytes. Final padding to reach 2048 bytes of ASCII text content for proper detection. Almost there now. Just a bit more text needed. This should be enough text data for the test to pass successfully. Done!";
        let data = text.as_bytes().to_vec();
        
        let patterns = analyzer.detect_patterns(&data);
        assert!(patterns.iter().any(|p| p.pattern_type == PatternType::Text));
    }

    #[test]
    fn test_compression_detection() {
        let analyzer = AiAnalyzer::new(512, 32);
        // Create varied data that won't be detected as repeating
        let mut data: Vec<u8> = (0..2048).map(|i| ((i * 7 + 13) % 256) as u8).collect();
        // gzip signature at start
        data[0] = 0x1F;
        data[1] = 0x8B;
        data[2] = 0x08;
        
        let patterns = analyzer.detect_patterns(&data);
        assert!(patterns.iter().any(|p| p.pattern_type == PatternType::Compressed));
    }

    #[test]
    fn test_repeating_pattern_detection() {
        let analyzer = AiAnalyzer::new(512, 32);
        let pattern = [0xAA, 0x55, 0xAA, 0x55];
        let data: Vec<u8> = pattern.iter().cycle().take(2048).copied().collect();
        
        let patterns = analyzer.detect_patterns(&data);
        assert!(patterns.iter().any(|p| p.pattern_type == PatternType::Repeating));
    }

    #[test]
    fn test_full_analysis() {
        let analyzer = AiAnalyzer::default();
        let mut data = vec![0xFFu8; 16384];
        
        // Add some compressed data
        data[0] = 0x1F;
        data[1] = 0x8B;
        data[2] = 0x08;
        
        // Add some text
        let text = b"Configuration file v1.0";
        data[4096..4096 + text.len()].copy_from_slice(text);
        
        let result = analyzer.analyze(&data);
        
        assert!(!result.patterns.is_empty());
        assert!(!result.summary.is_empty());
        assert!(result.data_quality_score >= 0.0 && result.data_quality_score <= 1.0);
    }

    #[test]
    fn test_confidence_conversion() {
        assert_eq!(Confidence::from_score(0.95), Confidence::VeryHigh);
        assert_eq!(Confidence::from_score(0.75), Confidence::High);
        assert_eq!(Confidence::from_score(0.55), Confidence::Medium);
        assert_eq!(Confidence::from_score(0.25), Confidence::Low);
    }

    #[test]
    fn test_entropy_calculation() {
        let analyzer = AiAnalyzer::default();
        
        // Uniform data = low entropy
        let uniform = vec![0xAAu8; 1000];
        let entropy = analyzer.calculate_entropy(&uniform);
        assert!(entropy < 0.1);
        
        // All different bytes = high entropy
        let varied: Vec<u8> = (0..=255).collect();
        let entropy = analyzer.calculate_entropy(&varied);
        assert!(entropy > 7.0);
    }
}
