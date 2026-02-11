using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using auth_service.Data;
using auth_service.Models;
using auth_service.Services;
using SharedContracts;
using System.ComponentModel.DataAnnotations;

namespace auth_service.Controllers;

[ApiController]
[Route("api/[controller]")]
public class AuthController : ControllerBase
{
    private readonly AuthDbContext _context;
    private readonly IAuthService _authService;

    public AuthController(AuthDbContext context, IAuthService authService)
    {
        _context = context;
        _authService = authService;
    }

    [HttpPost("register")]
    public async Task<IActionResult> Register([FromBody] RegisterRequest request)
    {
        if (!ModelState.IsValid)
            return BadRequest(ModelState);

        // Check if username or email already exists
        if (await _context.Users.AnyAsync(u => u.Username == request.Username))
            return BadRequest("Username already exists");

        if (await _context.Users.AnyAsync(u => u.Email == request.Email))
            return BadRequest("Email already exists");

        // Hash password
        var passwordHash = BCrypt.Net.BCrypt.HashPassword(request.Password);

        var user = new User
        {
            Username = request.Username,
            Email = request.Email,
            PasswordHash = passwordHash
        };

        _context.Users.Add(user);
        await _context.SaveChangesAsync();

        var token = _authService.GenerateJwtToken(user);
        var userDto = new UserDto { Id = user.Id, Username = user.Username, Email = user.Email };

        return Ok(new AuthResponse { Token = token, User = userDto });
    }

    [HttpPost("login")]
    public async Task<IActionResult> Login([FromBody] LoginRequest request)
    {
        if (!ModelState.IsValid)
            return BadRequest(ModelState);

        var user = await _context.Users.FirstOrDefaultAsync(u => u.Username == request.Username);
        if (user == null || !BCrypt.Net.BCrypt.Verify(request.Password, user.PasswordHash))
            return Unauthorized("Invalid username or password");

        var token = _authService.GenerateJwtToken(user);
        var userDto = new UserDto { Id = user.Id, Username = user.Username, Email = user.Email };

        return Ok(new AuthResponse { Token = token, User = userDto });
    }
}