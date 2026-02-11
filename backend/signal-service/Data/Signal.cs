using System.ComponentModel.DataAnnotations;

namespace signal_service.Data;

public class Signal
{
    [Key]
    public Guid Id { get; set; } = Guid.NewGuid();
    [Required]
    public string Symbol { get; set; }
    [Required]
    public string Type { get; set; }
    [Required]
    public decimal Price { get; set; }
    [Required]
    [Range(0, 1)]
    public decimal Confidence { get; set; }
    [Required]
    public DateTime Timestamp { get; set; }
}