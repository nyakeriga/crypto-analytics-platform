namespace SharedContracts;

public class UserDto
{
    public string Id { get; set; }
    public string Username { get; set; }
    public string Email { get; set; }
}

public class SignalDto
{
    public string Id { get; set; }
    public string Symbol { get; set; }
    public string Type { get; set; }
    public decimal Price { get; set; }
    public decimal Confidence { get; set; }
    public DateTime Timestamp { get; set; }
}

public class AdminDto
{
    public string Id { get; set; }
    public string Action { get; set; }
    public string Details { get; set; }
}

public class LoginRequest
{
    public string Username { get; set; }
    public string Password { get; set; }
}

public class RegisterRequest
{
    public string Username { get; set; }
    public string Email { get; set; }
    public string Password { get; set; }
}

public class AuthResponse
{
    public string Token { get; set; }
    public UserDto User { get; set; }
}
