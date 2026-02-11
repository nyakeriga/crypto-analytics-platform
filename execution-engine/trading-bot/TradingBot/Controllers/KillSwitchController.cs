using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace TradingBot.Controllers;

[ApiController]
[Route("api/[controller]")]
public class KillSwitchController : ControllerBase
{
    private readonly ILogger<KillSwitchController> _logger;
    private static bool _isTradingEnabled = true;

    public KillSwitchController(ILogger<KillSwitchController> logger)
    {
        _logger = logger;
    }

    [HttpGet("status")]
    public IActionResult GetStatus()
    {
        return Ok(new { isTradingEnabled = _isTradingEnabled });
    }

    [HttpPost("enable")]
    public IActionResult EnableTrading()
    {
        _isTradingEnabled = true;
        _logger.LogWarning("Trading has been ENABLED via kill switch");
        return Ok(new { message = "Trading enabled", isTradingEnabled = _isTradingEnabled });
    }

    [HttpPost("disable")]
    public IActionResult DisableTrading()
    {
        _isTradingEnabled = false;
        _logger.LogCritical("TRADING HAS BEEN DISABLED VIA KILL SWITCH - EMERGENCY STOP");
        return Ok(new { message = "Trading disabled", isTradingEnabled = _isTradingEnabled });
    }

    [HttpGet("health")]
    public IActionResult Health()
    {
        return Ok(new { status = "healthy", isTradingEnabled = _isTradingEnabled });
    }

    public static bool IsTradingEnabled()
    {
        return _isTradingEnabled;
    }
}