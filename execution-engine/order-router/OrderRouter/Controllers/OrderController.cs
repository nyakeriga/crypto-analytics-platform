using Microsoft.AspNetCore.Mvc;
using OrderRouter.Models;
using OrderRouter.Services;

namespace OrderRouter.Controllers;

[ApiController]
[Route("api/[controller]")]
public class OrderController : ControllerBase
{
    private readonly IOrderRouterService _orderRouterService;
    private readonly ILogger<OrderController> _logger;

    public OrderController(IOrderRouterService orderRouterService, ILogger<OrderController> logger)
    {
        _orderRouterService = orderRouterService;
        _logger = logger;
    }

    [HttpPost("place")]
    public async Task<IActionResult> PlaceOrder([FromBody] OrderRequest request)
    {
        if (!ModelState.IsValid)
        {
            return BadRequest(ModelState);
        }

        try
        {
            var response = await _orderRouterService.RouteOrderAsync(request);
            return Ok(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error placing order");
            return StatusCode(500, new { error = "Internal server error" });
        }
    }

    [HttpGet("health")]
    public IActionResult Health()
    {
        return Ok(new { status = "healthy" });
    }
}