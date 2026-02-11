using Microsoft.EntityFrameworkCore;

namespace signal_service.Data;

public class SignalDbContext : DbContext
{
    public SignalDbContext(DbContextOptions<SignalDbContext> options) : base(options) { }

    public DbSet<Signal> Signals { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<Signal>()
            .HasIndex(s => s.Symbol);
        modelBuilder.Entity<Signal>()
            .HasIndex(s => s.Timestamp);
    }
}