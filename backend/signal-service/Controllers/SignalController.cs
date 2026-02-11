using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using signal_service.Data;
using SharedContracts;
using System.ComponentModel.DataAnnotations;

namespace signal_service.Controllers;

[ApiController]
[Route("api/[controller]")]
public class SignalController : ControllerBase
{
    private readonly SignalDbContext _context;

    public SignalController(SignalDbContext context)
    {
        _context = context;
    }

    [HttpGet]
    public async Task<IActionResult> GetSignals(
        [FromQuery] string? symbol,
        [FromQuery] decimal? minConfidence,
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 10)
    {
        if (page < 1 || pageSize < 1 || pageSize > 100)
        {
            return BadRequest("Invalid pagination parameters.");
        }

        var query = _context.Signals.AsQueryable();

        if (!string.IsNullOrEmpty(symbol))
        {
            query = query.Where(s => s.Symbol == symbol);
        }

        if (minConfidence.HasValue)
        {
            query = query.Where(s => s.Confidence >= minConfidence.Value);
        }

        var total = await query.CountAsync();
        var signals = await query
            .OrderByDescending(s => s.Timestamp)
            .Skip((page - 1) * pageSize)
            .Take(pageSize)
            .Select(s => new SignalDto
            {
                Id = s.Id.ToString(),
                Symbol = s.Symbol,
                Type = s.Type,
                Price = s.Price,
                Confidence = s.Confidence,
                Timestamp = s.Timestamp
            })
            .ToListAsync();

        return Ok(new
        {
            Total = total,
            Page = page,
            PageSize = pageSize,
            Signals = signals
        });
    }

    [HttpPost("history")]
    public async Task<IActionResult> PostSignalHistory([FromBody] SignalDto signalDto)
    {
        if (!ModelState.IsValid)
        {
            return BadRequest(ModelState);
        }

        // Validate signal
        if (string.IsNullOrEmpty(signalDto.Symbol) || signalDto.Confidence < 0 || signalDto.Confidence > 1)
        {
            return BadRequest("Invalid signal data.");
        }

        var signal = new Signal
        {
            Symbol = signalDto.Symbol,
            Type = signalDto.Type,
            Price = signalDto.Price,
            Confidence = signalDto.Confidence,
            Timestamp = signalDto.Timestamp
        };

        _context.Signals.Add(signal);
        await _context.SaveChangesAsync();

        return CreatedAtAction(nameof(GetSignals), new { id = signal.Id }, signalDto);
    }
}