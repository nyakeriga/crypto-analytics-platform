using Yarp.ReverseProxy.Configuration;
using ApiGateway.Services;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

// JWT Authentication
builder.Services.AddAuthentication("Bearer")
    .AddJwtBearer("Bearer", options =>
    {
        options.Authority = "https://your-auth-server"; // Replace with actual auth server
        options.Audience = "api-gateway";
        options.RequireHttpsMetadata = false; // For development
    });

// Rate Limiting - simplified
builder.Services.AddRateLimiter(_ => {});

// Add services
builder.Services.AddHostedService<SignalWebSocketService>();

// Reverse Proxy for services
builder.Services.AddReverseProxy()
    .LoadFromMemory(new[]
    {
        new RouteConfig
        {
            RouteId = "auth-service",
            ClusterId = "auth-cluster",
            Match = new RouteMatch { Path = "/auth/{**catch-all}" }
        },
        new RouteConfig
        {
            RouteId = "signal-service",
            ClusterId = "signal-cluster",
            Match = new RouteMatch { Path = "/signals/{**catch-all}" }
        },
        new RouteConfig
        {
            RouteId = "user-service",
            ClusterId = "user-cluster",
            Match = new RouteMatch { Path = "/users/{**catch-all}" }
        },
        new RouteConfig
        {
            RouteId = "admin-service",
            ClusterId = "admin-cluster",
            Match = new RouteMatch { Path = "/admin/{**catch-all}" }
        }
    }, new[]
    {
        new ClusterConfig
        {
            ClusterId = "auth-cluster",
            Destinations = new Dictionary<string, DestinationConfig>
            {
                ["auth-destination"] = new DestinationConfig { Address = "http://localhost:5001" } // Replace with actual
            }
        },
        new ClusterConfig
        {
            ClusterId = "signal-cluster",
            Destinations = new Dictionary<string, DestinationConfig>
            {
                ["signal-destination"] = new DestinationConfig { Address = "http://localhost:5002" }
            }
        },
        new ClusterConfig
        {
            ClusterId = "user-cluster",
            Destinations = new Dictionary<string, DestinationConfig>
            {
                ["user-destination"] = new DestinationConfig { Address = "http://localhost:5003" }
            }
        },
        new ClusterConfig
        {
            ClusterId = "admin-cluster",
            Destinations = new Dictionary<string, DestinationConfig>
            {
                ["admin-destination"] = new DestinationConfig { Address = "http://localhost:5004" }
            }
        }
    });

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.UseWebSockets();

app.UseAuthentication();
app.UseAuthorization();

app.UseRateLimiter();

app.MapControllers();
app.MapReverseProxy();

app.Run();
