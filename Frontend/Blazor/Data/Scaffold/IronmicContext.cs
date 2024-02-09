using Microsoft.EntityFrameworkCore;

namespace Blazor.Data.Scaffold;

public partial class IronmicContext : DbContext
{
    public IronmicContext()
    {
    }

    public IronmicContext(DbContextOptions<IronmicContext> options)
        : base(options)
    {
    }

    public virtual DbSet<ActiveControllerSession> ActiveControllerSessions { get; set; }

    public virtual DbSet<ActivePositionSession> ActivePositionSessions { get; set; }

    public virtual DbSet<Artcc> Artccs { get; set; }

    public virtual DbSet<CompletedControllerSession> CompletedControllerSessions { get; set; }

    public virtual DbSet<CompletedPositionSession> CompletedPositionSessions { get; set; }

    public virtual DbSet<ControllerSessionPositionJoin> ControllerSessionPositionJoins { get; set; }

    public virtual DbSet<IronMic.DatafeedRecord> DatafeedRecords { get; set; }

    public virtual DbSet<Facility> Facilities { get; set; }

    public virtual DbSet<Position> Positions { get; set; }

    public virtual DbSet<PositionSessionFacilityJoin> PositionSessionFacilityJoins { get; set; }

    public virtual DbSet<SqlxMigration> SqlxMigrations { get; set; }

    public virtual DbSet<VnasFetchRecord> VnasFetchRecords { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
#warning To protect potentially sensitive information in your connection string, you should move it out of source code. You can avoid scaffolding the connection string by using the Name= syntax to read it from configuration - see https: //go.microsoft.com/fwlink/?linkid=2131148. For more guidance on storing connection strings, see https://go.microsoft.com/fwlink/?LinkId=723263.
        => optionsBuilder.UseNpgsql("Host=localhost:5432;Username=postgres;Password=pw;Database=ironmic");

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<ActiveControllerSession>(entity =>
        {
            entity.HasKey(e => new { e.Id, e.IsActive }).HasName("active_controller_sessions_pkey");

            entity.ToTable("active_controller_sessions");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.IsActive).HasColumnName("is_active");
            entity.Property(e => e.AssocVnasPositions)
                .HasColumnType("jsonb")
                .HasColumnName("assoc_vnas_positions");
            entity.Property(e => e.Cid).HasColumnName("cid");
            entity.Property(e => e.ConnectedCallsign).HasColumnName("connected_callsign");
            entity.Property(e => e.ConnectedFrequency).HasColumnName("connected_frequency");
            entity.Property(e => e.DatafeedFirst).HasColumnName("datafeed_first");
            entity.Property(e => e.DatafeedLast).HasColumnName("datafeed_last");
            entity.Property(e => e.Duration).HasColumnName("duration");
            entity.Property(e => e.EndTime).HasColumnName("end_time");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.PositionSessionId).HasColumnName("position_session_id");
            entity.Property(e => e.PositionSessionIsActive).HasColumnName("position_session_is_active");
            entity.Property(e => e.PositionSimpleCallsign).HasColumnName("position_simple_callsign");
            entity.Property(e => e.StartTime).HasColumnName("start_time");
        });

        modelBuilder.Entity<ActivePositionSession>(entity =>
        {
            entity.HasKey(e => new { e.Id, e.IsActive }).HasName("active_position_sessions_pkey");

            entity.ToTable("active_position_sessions");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.IsActive).HasColumnName("is_active");
            entity.Property(e => e.AssocVnasFacilities)
                .HasColumnType("jsonb")
                .HasColumnName("assoc_vnas_facilities");
            entity.Property(e => e.DatafeedFirst).HasColumnName("datafeed_first");
            entity.Property(e => e.DatafeedLast).HasColumnName("datafeed_last");
            entity.Property(e => e.Duration).HasColumnName("duration");
            entity.Property(e => e.EndTime).HasColumnName("end_time");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.PositionSimpleCallsign).HasColumnName("position_simple_callsign");
            entity.Property(e => e.StartTime).HasColumnName("start_time");
        });

        modelBuilder.Entity<Artcc>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("artccs_pkey");

            entity.ToTable("artccs");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
        });

        modelBuilder.Entity<CompletedControllerSession>(entity =>
        {
            entity.HasKey(e => new { e.Id, e.IsActive }).HasName("completed_controller_sessions_pkey");

            entity.ToTable("completed_controller_sessions");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.IsActive).HasColumnName("is_active");
            entity.Property(e => e.AssocVnasPositions)
                .HasColumnType("jsonb")
                .HasColumnName("assoc_vnas_positions");
            entity.Property(e => e.Cid).HasColumnName("cid");
            entity.Property(e => e.ConnectedCallsign).HasColumnName("connected_callsign");
            entity.Property(e => e.ConnectedFrequency).HasColumnName("connected_frequency");
            entity.Property(e => e.DatafeedFirst).HasColumnName("datafeed_first");
            entity.Property(e => e.DatafeedLast).HasColumnName("datafeed_last");
            entity.Property(e => e.Duration).HasColumnName("duration");
            entity.Property(e => e.EndTime).HasColumnName("end_time");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.PositionSessionId).HasColumnName("position_session_id");
            entity.Property(e => e.PositionSessionIsActive).HasColumnName("position_session_is_active");
            entity.Property(e => e.PositionSimpleCallsign).HasColumnName("position_simple_callsign");
            entity.Property(e => e.StartTime).HasColumnName("start_time");
        });

        modelBuilder.Entity<CompletedPositionSession>(entity =>
        {
            entity.HasKey(e => new { e.Id, e.IsActive }).HasName("completed_position_sessions_pkey");

            entity.ToTable("completed_position_sessions");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.IsActive).HasColumnName("is_active");
            entity.Property(e => e.AssocVnasFacilities)
                .HasColumnType("jsonb")
                .HasColumnName("assoc_vnas_facilities");
            entity.Property(e => e.DatafeedFirst).HasColumnName("datafeed_first");
            entity.Property(e => e.DatafeedLast).HasColumnName("datafeed_last");
            entity.Property(e => e.Duration).HasColumnName("duration");
            entity.Property(e => e.EndTime).HasColumnName("end_time");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.PositionSimpleCallsign).HasColumnName("position_simple_callsign");
            entity.Property(e => e.StartTime).HasColumnName("start_time");
        });

        modelBuilder.Entity<ControllerSessionPositionJoin>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("controller_session_position_join_pkey");

            entity.ToTable("controller_session_position_join");

            entity.Property(e => e.Id)
                .UseIdentityAlwaysColumn()
                .HasColumnName("id");
            entity.Property(e => e.ControllerSessionId).HasColumnName("controller_session_id");
            entity.Property(e => e.ControllerSessionIsActive).HasColumnName("controller_session_is_active");
            entity.Property(e => e.FrozenData)
                .HasColumnType("jsonb")
                .HasColumnName("frozen_data");
            entity.Property(e => e.PositionId).HasColumnName("position_id");
            entity.Property(e => e.PositionParentFacilityId).HasColumnName("position_parent_facility_id");

            entity.HasOne(d => d.ActiveControllerSession).WithMany(p => p.ControllerSessionPositionJoins)
                .HasForeignKey(d => new { d.ControllerSessionId, d.ControllerSessionIsActive })
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("controller_session_position__controller_session_id_contro_fkey1");

            entity.HasOne(d => d.CompletedControllerSession).WithMany(p => p.ControllerSessionPositionJoins)
                .HasForeignKey(d => new { d.ControllerSessionId, d.ControllerSessionIsActive })
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("controller_session_position__controller_session_id_contro_fkey2");

            entity.HasOne(d => d.Position).WithMany(p => p.ControllerSessionPositionJoins)
                .HasForeignKey(d => new { d.PositionId, d.PositionParentFacilityId })
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("controller_session_position_j_position_id_position_parent__fkey");
        });

        modelBuilder.Entity<IronMic.DatafeedRecord>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("datafeed_records_pkey");

            entity.ToTable("datafeed_records");

            entity.Property(e => e.Id)
                .UseIdentityAlwaysColumn()
                .HasColumnName("id");
            entity.Property(e => e.NumTrackedControllerSessions).HasColumnName("num_tracked_controller_sessions");
            entity.Property(e => e.NumTrackedPositionSessions).HasColumnName("num_tracked_position_sessions");
            entity.Property(e => e.UpdateTimestamp).HasColumnName("update");
        });

        modelBuilder.Entity<Facility>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("facilities_pkey");

            entity.ToTable("facilities");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.Name).HasColumnName("name");
            entity.Property(e => e.ParentArtccId).HasColumnName("parent_artcc_id");
            entity.Property(e => e.ParentFacilityId).HasColumnName("parent_facility_id");
            entity.Property(e => e.Type).HasColumnName("type");

            entity.HasOne(d => d.ParentArtcc).WithMany(p => p.Facilities)
                .HasForeignKey(d => d.ParentArtccId)
                .HasConstraintName("facilities_parent_artcc_id_fkey");

            entity.HasOne(d => d.ParentFacility).WithMany(p => p.InverseParentFacility)
                .HasForeignKey(d => d.ParentFacilityId)
                .HasConstraintName("facilities_parent_facility_id_fkey");
        });

        modelBuilder.Entity<Position>(entity =>
        {
            entity.HasKey(e => new { e.Id, e.ParentFacilityId }).HasName("positions_pkey");

            entity.ToTable("positions");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.ParentFacilityId).HasColumnName("parent_facility_id");
            entity.Property(e => e.Callsign).HasColumnName("callsign");
            entity.Property(e => e.CallsignInfix).HasColumnName("callsign_infix");
            entity.Property(e => e.CallsignPrefix).HasColumnName("callsign_prefix");
            entity.Property(e => e.CallsignSuffix).HasColumnName("callsign_suffix");
            entity.Property(e => e.CallsignWithoutInfix).HasColumnName("callsign_without_infix");
            entity.Property(e => e.Frequency).HasColumnName("frequency");
            entity.Property(e => e.LastUpdated).HasColumnName("last_updated");
            entity.Property(e => e.Name).HasColumnName("name");
            entity.Property(e => e.RadioName).HasColumnName("radio_name");
            entity.Property(e => e.Starred).HasColumnName("starred");

            entity.HasOne(d => d.ParentFacility).WithMany(p => p.Positions)
                .HasForeignKey(d => d.ParentFacilityId)
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("positions_parent_facility_id_fkey");
        });

        modelBuilder.Entity<PositionSessionFacilityJoin>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("position_session_facility_join_pkey");

            entity.ToTable("position_session_facility_join");

            entity.Property(e => e.Id)
                .UseIdentityAlwaysColumn()
                .HasColumnName("id");
            entity.Property(e => e.FacilityId).HasColumnName("facility_id");
            entity.Property(e => e.FrozenData)
                .HasColumnType("jsonb")
                .HasColumnName("frozen_data");
            entity.Property(e => e.PositionSessionId).HasColumnName("position_session_id");
            entity.Property(e => e.PositionSessionIsActive).HasColumnName("position_session_is_active");

            entity.HasOne(d => d.Facility).WithMany(p => p.PositionSessionFacilityJoins)
                .HasForeignKey(d => d.FacilityId)
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("position_session_facility_join_facility_id_fkey");

            entity.HasOne(d => d.ActivePositionSession).WithMany(p => p.PositionSessionFacilityJoins)
                .HasForeignKey(d => new { d.PositionSessionId, d.PositionSessionIsActive })
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("position_session_facility_jo_position_session_id_position_fkey2");

            entity.HasOne(d => d.CompletedPositionSession).WithMany(p => p.PositionSessionFacilityJoins)
                .HasForeignKey(d => new { d.PositionSessionId, d.PositionSessionIsActive })
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("position_session_facility_jo_position_session_id_position_fkey1");
        });

        modelBuilder.Entity<SqlxMigration>(entity =>
        {
            entity.HasKey(e => e.Version).HasName("_sqlx_migrations_pkey");

            entity.ToTable("_sqlx_migrations");

            entity.Property(e => e.Version)
                .ValueGeneratedNever()
                .HasColumnName("version");
            entity.Property(e => e.Checksum).HasColumnName("checksum");
            entity.Property(e => e.Description).HasColumnName("description");
            entity.Property(e => e.ExecutionTime).HasColumnName("execution_time");
            entity.Property(e => e.InstalledOn)
                .HasDefaultValueSql("now()")
                .HasColumnName("installed_on");
            entity.Property(e => e.Success).HasColumnName("success");
        });

        modelBuilder.Entity<VnasFetchRecord>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("vnas_fetch_records_pkey");

            entity.ToTable("vnas_fetch_records");

            entity.Property(e => e.Id)
                .UseIdentityAlwaysColumn()
                .HasColumnName("id");
            entity.Property(e => e.Success).HasColumnName("success");
            entity.Property(e => e.UpdateTime).HasColumnName("update_time");
        });

        OnModelCreatingPartial(modelBuilder);
    }

    partial void OnModelCreatingPartial(ModelBuilder modelBuilder);
}