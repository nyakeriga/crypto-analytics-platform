using auth_service.Models;

namespace auth_service.Services;

public interface IAuthService
{
    string GenerateJwtToken(User user);
}