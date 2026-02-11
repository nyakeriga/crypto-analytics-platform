using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using SharedContracts;
using user_service.Data;
using user_service.Models;

namespace user_service.Controllers;

[ApiController]
[Route("api/users")]
public class UserController : ControllerBase
{
    private readonly AppDbContext _context;

    public UserController(AppDbContext context)
    {
        _context = context;
    }

    [HttpGet("profile")]
    public async Task<ActionResult<UserDto>> GetProfile()
    {
        var userIdHeader = Request.Headers["X-User-Id"].ToString();
        if (string.IsNullOrEmpty(userIdHeader) || !Guid.TryParse(userIdHeader, out var userId))
        {
            return BadRequest("Invalid or missing X-User-Id header");
        }

        var user = await _context.Users.FindAsync(userId);
        if (user == null)
        {
            return NotFound("User not found");
        }

        var userDto = new UserDto
        {
            Id = user.Id.ToString(),
            Username = user.Username,
            Email = user.Email
        };

        return Ok(userDto);
    }

    [HttpPut("profile")]
    public async Task<IActionResult> UpdateProfile([FromBody] UserDto userDto)
    {
        if (!ModelState.IsValid)
        {
            return BadRequest(ModelState);
        }

        var userIdHeader = Request.Headers["X-User-Id"].ToString();
        if (string.IsNullOrEmpty(userIdHeader) || !Guid.TryParse(userIdHeader, out var userId))
        {
            return BadRequest("Invalid or missing X-User-Id header");
        }

        var user = await _context.Users.FindAsync(userId);
        if (user == null)
        {
            return NotFound("User not found");
        }

        // Update fields
        user.Username = userDto.Username;
        user.Email = userDto.Email;
        user.UpdatedAt = DateTime.UtcNow;

        try
        {
            await _context.SaveChangesAsync();
        }
        catch (DbUpdateException)
        {
            return Conflict("Email already exists");
        }

        return NoContent();
    }

    [HttpGet("subscription")]
    public async Task<ActionResult<SubscriptionDto>> GetSubscription()
    {
        var userIdHeader = Request.Headers["X-User-Id"].ToString();
        if (string.IsNullOrEmpty(userIdHeader) || !Guid.TryParse(userIdHeader, out var userId))
        {
            return BadRequest("Invalid or missing X-User-Id header");
        }

        var subscription = await _context.Subscriptions
            .Where(s => s.UserId == userId)
            .OrderByDescending(s => s.StartDate)
            .FirstOrDefaultAsync();

        if (subscription == null)
        {
            return NotFound("No subscription found");
        }

        var subscriptionDto = new SubscriptionDto
        {
            Id = subscription.Id.ToString(),
            Plan = subscription.Plan,
            StartDate = subscription.StartDate,
            EndDate = subscription.EndDate,
            Status = subscription.Status
        };

        return Ok(subscriptionDto);
    }
}

public class SubscriptionDto
{
    public string Id { get; set; }
    public string Plan { get; set; }
    public DateTime StartDate { get; set; }
    public DateTime? EndDate { get; set; }
    public string Status { get; set; }
}