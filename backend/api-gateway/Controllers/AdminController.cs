using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using SharedContracts;

namespace ApiGateway.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize(Roles = "Admin")]
public class AdminController : ControllerBase
{
    [HttpPost("action")]
    public ActionResult<AdminDto> PerformAction([FromBody] AdminDto action)
    {
        // Mock
        return action;
    }
}