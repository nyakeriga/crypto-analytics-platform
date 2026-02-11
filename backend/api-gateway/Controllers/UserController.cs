using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using SharedContracts;

namespace ApiGateway.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class UserController : ControllerBase
{
    [HttpGet("profile")]
    public ActionResult<UserDto> GetProfile()
    {
        // Mock or proxy
        return new UserDto { Id = "1", Username = "user", Email = "user@example.com" };
    }
}